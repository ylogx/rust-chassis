use std::{thread, time};
extern crate rand;
use rand::Rng;
use tracing::debug;

pub fn wait_for_about(num_secs: u64) -> u64 {
    let lower_in_ms = (num_secs - (num_secs / 2)) * 1000;
    let upper_in_ms = num_secs * 1000;
    let mut rng = rand::thread_rng();
    let noisy_duration_in_ms = rng.gen_range(lower_in_ms..upper_in_ms);

    debug!("Sleeping for {noisy_duration_in_ms} ms");
    let wait_duration = time::Duration::from_millis(noisy_duration_in_ms);
    let now = time::Instant::now();
    thread::sleep(wait_duration);

    assert!(now.elapsed() >= wait_duration);
    now.elapsed().as_secs()
}
