use ron::de::from_reader;
use std::{collections::HashMap, env, error::Error, fs::File};

mod load;
mod models;

use crate::load::run_load_test;
use models::Config;

/// Loads the config.ron file into a struct
fn load_config() -> Result<Config, Box<dyn Error>> {
    let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path)?;
    let config: Config = from_reader(f)?;

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load config file
    let config = load_config()?;
    run_load_test(&config).await?;

    Ok(())
}
