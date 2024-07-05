#![allow(unused_parens)]

mod config;
mod constants;
mod helpers;
mod runtime;

use std::fs;

use constants::{ID, NAME, PLUGIN};
use expressrs::plugin::BasePlugin;
use runtime::checker::check;
use runtime::copier::{clean, copy, run};

pub fn svelte() -> BasePlugin {
    BasePlugin {
        name: NAME.to_string(),
        id: ID.to_string(),
        plugin_id: PLUGIN.to_string(),
        dist: "dist".to_string(),
        features: None,
        main: Box::new(|public_dir| {
            let root_dir = env!("CARGO_MANIFEST_DIR");

            clean();

            check();
            copy(public_dir, format!("{}/build/src/svelte", root_dir));
            run();

            fs::create_dir_all("dist").expect("Failed to create a dist directory");
            copy(format!("{}/build/dist", root_dir), "dist".to_string());
        }),
    }
}
