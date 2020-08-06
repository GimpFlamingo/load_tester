use ron::de::from_reader;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    time::Instant,
};

mod load;
mod models;
mod statistics;

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
    let start_time = Instant::now();
    let mut load_results = run_load_test(&config).await?;
    let end_time = Instant::now();
    load_results.total_time = end_time.duration_since(start_time).as_secs_f64();

    Ok(())
}
