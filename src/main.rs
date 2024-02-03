mod bizness;
mod logging;
mod noisy_waiter;

use clap::Parser;
use tracing::debug;
use tracing::info;

const DEFAULT_WAIT_SECS: u64 = 5;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    monitor: bool,

    #[arg(short, long, default_value_t = DEFAULT_WAIT_SECS)]
    diff: u64,
}

fn main() {
    let args = Args::parse();
    args_validation(&args);
    logging::init_logger();

    info!("Starting...");
    loop {
        debug!("Task started");
        bizness::performance();
        info!("Business performance audited.");

        if args.monitor {
            debug!("Running every {} secs", args.diff);
            let waited: u64 = noisy_waiter::wait_for_about(args.diff);
            debug!("Waited for {waited} secs");
        } else {
            break;
        }
    }
}

fn args_validation(args: &Args) {
    if args.monitor && args.diff <= 1 {
        panic!("The diff must be at least 1 second when monitoring");
    }
}
