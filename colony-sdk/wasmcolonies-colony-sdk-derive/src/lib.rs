use proc_macro::TokenStream;
use quote::quote;

/// Marks the function that handles game engine ticks
///
/// # Examples
/// ```
/// #[colony::tick]
/// fn tick() {
///     // Register message handlers...
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
#[proc_macro_attribute]
pub fn tick(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;

    let sig = &mut input.sig;
    let body = &input.block;

    if sig.asyncness.is_some() {
        return syn::Error::new_spanned(
            sig.fn_token,
            "the async keyword cannot be used within actors",
        )
        .to_compile_error()
        .into();
    }

    sig.asyncness = None;

    (quote! {
        use wasmcolonies_protocol as protocol;
        use wasmcolonies_colony_sdk as sdk;
        use wasmcloud_actor_core;
        use wapc_guest;

        #[doc(hidden)]
        fn default_health(_msg: wasmcloud_actor_core::HealthCheckRequest) -> wapc_guest::HandlerResult<wasmcloud_actor_core::HealthCheckResponse> {
            Ok(wasmcloud_actor_core::HealthCheckResponse::healthy())
        }

        #(#attrs)*
        #[no_mangle]
        pub fn wapc_init() {
            wasmcloud_actor_core::Handlers::register_health_request(default_health);
            sdk::Handlers::register_player_tick(colony_tick);            
        }

        #[doc(hidden)]
        fn colony_tick(tick: protocol::PlayerTick) -> wapc_guest::HandlerResult<protocol::PlayerTickResponse> {
            #body

            // TODO
            Ok(protocol::PlayerTickResponse{
                commands: vec![]
            })
        }
    })
    .into()
}
