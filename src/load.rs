use rand::{seq::SliceRandom, thread_rng};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    time,
    time::Instant,
};
use tokio::task;
use futures::future::join_all;

use crate::error::Result;
use crate::models;
use crate::models::{RequestResult, LoadResults};
use tokio::time::Duration;

/// Arc mutex of a Vec<RequestResult>
type RR = Arc<Mutex<Vec<RequestResult>>>;

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
async fn perform_request(url: String) -> Result<models::RequestResult> {
    // Start timer
    let start = Instant::now();
    // Get the http response from the url
    let res = reqwest::get(&url).await?;
    // End timer
    let end = Instant::now();
    if !res.status().is_success() {
        println!("{:#?}", res);
    }

    Ok(RequestResult::new(
        &url,
        res.status().as_u16(),
        end.duration_since(start),
        0,
    ))
}

async fn start_runs(urls: Vec<String>) -> Result<Vec<RequestResult>> {
    let mut requests = vec![];
    for url in urls {
        let url = url.clone();
        let response = tokio::spawn(perform_request(url));
    }

    Ok(requests)
}

async fn perform_runs(config: &models::Config) -> Result<LoadResults> {
    let mut load_results = LoadResults::new();
    let mut run_futures = vec![];
    for run in 0..config.runs {
        let urls = shuffle(&config.urls);
        // Spawn a task for each run
        let result = tokio::spawn(async move {
            let mut futures = vec![];
            // Spawn a task for each url
            for url in urls {
                let url = url.clone();
                // let response = tokio::spawn(perform_request(url));
                futures.push(tokio::spawn(perform_request(url)));
            }
            let mut to_return = vec![];
            let url_future_result = join_all(futures).await;
            let mut iter = url_future_result.iter();
            // Iterate through the future results of the url requests and parse out the Request result that is wrapped inside two Results
            while let Some(url_result) = iter.next() {
                if let Ok(first_res) = url_result {
                    match first_res {
                        Ok(wrapped_result_value) => to_return.push(wrapped_result_value.clone()),
                        Err(_) => (),
                    };
                }
            }
            to_return
        });
        run_futures.push(result);
    }
    let final_results = join_all(run_futures).await;
    let mut iter = final_results.iter();
    while let Some(run_val) = iter.next() {
        match run_val {
            Ok(request_result_vec) => {
                load_results.add_results(request_result_vec.clone());
            },
            Err(_) => (),
        }
    }
    // for res in final_results {
    //     simplify.push(res)
    // }

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
    // let load_results = task::spawn(async move { perform_runs(&config).await.unwrap() });
    let load_results = perform_runs(&config);
    // Wait for each request to finish
    Ok(load_results.await?)
}
