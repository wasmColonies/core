# Shard Driver

Totally experimental.

Right now, to see this work:

terminal 1: `wasmcloud -m ./manifest.yaml` (in the **noop** actor directory after building and signing)

terminal 2: `cargo run` (note you'll have to edit the code to change the public key of the target actor)

Currently taking 1 microsecond (`.001s`) per tick on localhost.
