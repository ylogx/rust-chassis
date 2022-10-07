use tracing::info;

pub fn init_logger() {
    let logger_builder = tracing_subscriber::fmt().compact();
    let debugging = false; // TODO: Use env variable to parse this
    if debugging {
        logger_builder
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(true)
            .with_max_level(tracing::Level::TRACE)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .init();
        info!("Set up debugging logger")
    } else {
        logger_builder
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .init();
        info!("Set up release logger")
    }
}
