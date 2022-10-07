mod bizness;
mod logging;
mod noisy_waiter;

use tracing::debug;
use tracing::info;

const DEFAULT_WAIT_SECS: u64 = 5;

fn main() {
    logging::init_logger();

    info!("Running every {DEFAULT_WAIT_SECS} secs");
    loop {
        debug!("Task started");
        bizness::performance();
        info!("Buisness performance audited.");

        let waited: u64 = noisy_waiter::wait_for_about(DEFAULT_WAIT_SECS);
        debug!("Waited for {waited} secs");
    }
}
