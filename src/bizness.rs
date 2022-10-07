extern crate rand;
use rand::Rng;
use tracing::debug;

pub fn performance() {
    if rand::random() {
        perform_something();
    } else {
        debug!("No action taken this time");
    }
}

fn perform_something() {
    let lower = 9.0;
    let upper = 9099.0;
    debug!("Performing the move ({lower}, {upper}))");
    let mut rng = rand::thread_rng();
    let noiseness = rng.gen_range(lower..upper);

    let steps = upper + lower / 2. - noiseness;
    debug!("Move ({lower}, {upper}) in {steps} steps)");
}
