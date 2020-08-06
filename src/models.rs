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
    pub did_succeed: bool,
    pub duration: Duration,
}

impl RequestResult {
    pub fn new(url: &str, status: u32, duration: Duration) -> RequestResult {
        RequestResult {
            url: String::from(url),
            status,
            did_succeed: if status > 199 && status < 300 {
                true
            } else {
                false
            },
            duration,
        }
    }
}

/// Represents the results for the entire load test
pub struct LoadResults {
    pub total_requests: usize,
    pub successes: usize,
    pub failures: usize,
    pub total_time: f64,
    pub mean_request_time: f64,
    pub median_request_time: f64,
    pub results: Vec<RequestResult>,
}

impl LoadResults {
    pub fn new(total_run_time: f64, mean: f64, median: f64, results_vec: Vec<RequestResult>) -> LoadResults {
        let (mut successes_count, mut failures_count) = (0, 0);
        for res in results_vec.iter() {
            match res.did_succeed {
                true => success += 1,
                false => failures_count += 1,
            }
        }

        LoadResults {
            total_requests: results_vec.len(),
            successes: successes_count,
            failures: failures_count,
            total_time: total_run_time,
            mean_request_time: mean,
            median_request_time: median,
            results: results_vec
        }
    }
}
