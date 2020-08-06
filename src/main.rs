use ron::de::from_reader;
use std::{
    fs::File,
    env,
    error::Error,
};

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path)?;
    let config: models::Config = from_reader(f)?;

    println!("Config: {:?}", &config);

    Ok(())
}
