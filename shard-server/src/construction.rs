use bevy::prelude::*;

pub struct ConstructionSite {
    pub progress: u8,
    pub progress_rate_pct: u8,
}

pub fn construction(mut query: Query<&mut ConstructionSite>) {
    for mut site in query.iter_mut() {
        let old = site.progress;
        site.progress = site
            .progress
            .saturating_add(site.progress_rate_pct)
            .min(100);
        if old < site.progress && site.progress == 100 {
            info!("Construction site completed.");
        }
    }
}
