use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub urls: Vec<String>,
    pub runs: i32,
}

#[derive(Debug)]
pub struct RequestResult {
    pub url: String,
    pub status: u32,
    pub duration: Duration,
}

pub struct LoadResults {
    pub total_requests: usize,
    pub successes: usize,
    pub failures: usize,
    pub total_time: f64,
    pub mean_request_time: f64,
    pub median_request_time: f64,
}
