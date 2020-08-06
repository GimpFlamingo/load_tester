use rand::{seq::SliceRandom, thread_rng};
use std::time::Instant;
use std::{error::Error, time};
use tokio::task;

use crate::error::Result;
use crate::models;
use crate::models::RequestResult;
use tokio::time::Duration;

/// Shuffles a vector and returns a new vector
fn shuffle<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    new_vec.shuffle(&mut thread_rng());

    new_vec
}

/// Performs a get request at the given url
///
/// Performs a get request to the given url and parses the relevant information into a RequestResult
/// struct to be returned.
async fn perform_request(url: &str) -> Result<models::RequestResult> {
    // Start timer
    let start = Instant::now();
    // Get the http response from the url
    let res = reqwest::get(url).await?;
    // End timer
    let end = Instant::now();
    if !res.status().is_success() {
        println!("{:#?}", res);
    }

    Ok(RequestResult::new(
        url,
        res.status().as_u16(),
        end.duration_since(start),
        0,
    ))
}

async fn build_request(url: String, run: usize) -> Result<RequestResult> {
    Ok(task::spawn(async move {
        // Make the requests
        let result = perform_request(&url).await;
        // Save results into struct
        match result {
            Ok(mut x) => {
                x.run = run;
                x
            }
            Err(e) => {
                println!("Something went wrong with URL: {}\nError: {:#?}", &url, e);
                RequestResult::new(&url, 500, Duration::new(0, 0), 0 as usize)
            }
        }
    })
    .await?)
}

async fn perform_runs(config: &models::Config) -> Result<models::LoadResults> {
    let mut requests = vec![];
    let mut load_results = models::LoadResults::new();
    for run in 0..config.runs {
        // Before each run need to shuffle the urls. Try and pause timer before here
        let urls = shuffle(&config.urls);
        // For each url make a request
        for url in urls {
            requests.push(build_request(url, run as usize).await.unwrap())
        }
        for request in requests.drain(..) {
            load_results.add_result(request);
        }
    }

    Ok(load_results)
}

/// Runs the load tests
///
/// Takes the given config struct loaded from the config file and loops through all the urls.
/// `config.runs` times, At the beginning of each loop, all the urls will be randomly shuffled. Then,
/// a get request will be made to each url with their results being saved to a `LoadResults` struct
/// and returned.
pub async fn run_load_test(config_orig: &models::Config) -> Result<models::LoadResults> {
    let config = config_orig.clone();
    let load_results = task::spawn(async move { perform_runs(&config).await.unwrap() });
    // Wait for each request to finish
    Ok(load_results.await?)
}
