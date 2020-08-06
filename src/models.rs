use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub urls: Vec<String>,
    pub runs: i32,
}