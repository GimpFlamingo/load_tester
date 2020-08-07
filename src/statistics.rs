use crate::models::LoadResults;

pub fn calculate_statistics(load_results: &mut LoadResults) {
    let mut total = 0.0;

    for result in load_results.results.to_vec() {
        total += result.duration;
    }

    load_results.mean_request_time = total / (load_results.results.len() as f64);
    load_results.median_request_time =
        load_results.results[load_results.results.len() / 2].duration;
}
