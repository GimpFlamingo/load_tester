use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    urls: Vec<String>,
    runs: i32,
}