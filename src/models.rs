use serde::Deserialize;
use std::time::Duration;

/// Models the configuration provided in config.ron
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub urls: Vec<String>,
    pub runs: usize,
}

/// Models the needed information from a request
#[derive(Debug, Clone)]
pub struct RequestResult {
    pub url: String,
    pub status: u16,
    pub did_succeed: bool,
    pub duration: f64,
    pub run: usize,
}

impl RequestResult {
    /// Creates a new RequestResult
    ///
    /// Takes the given values and returns a fully initialized RequestResult
    ///
    /// # Examples
    ///```
    /// let request_result = RequestResult::new("localhost:5001", 200, Duration::from_secs(2,0));
    ///```
    pub fn new(url: &str, status: u16, dur: Duration, run_num: usize) -> RequestResult {
        RequestResult {
            url: String::from(url),
            status,
            did_succeed: status > 199 && status < 300,
            duration: dur.as_secs_f64(),
            run: run_num,
        }
    }
}

/// Represents the results for the entire load test
#[derive(Debug, Clone)]
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
    /// Creates a new LoadResults with default values
    ///
    /// # Examples
    /// ```
    /// let load_results = LoadResults::new();
    /// ```
    pub fn new() -> LoadResults {
        LoadResults {
            total_requests: 0,
            successes: 0,
            failures: 0,
            total_time: 0.0,
            mean_request_time: 0.0,
            median_request_time: 0.0,
            results: vec![],
        }
    }

    /// Adds a vector of results to the struct
    ///
    /// Adds a vector of result to LoadResults. It will also increment total_requests and success
    /// or failures based on the information provided from each RequestResult
    pub fn add_results(&mut self, results: Vec<RequestResult>) {
        for res in results {
            self.total_requests += 1;
            match res.did_succeed {
                true => self.successes += 1,
                false => self.failures += 1,
            }
            self.results.push(res);
        }
    }
}
