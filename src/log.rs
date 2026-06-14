use tracing::Level;

pub fn init_log(log_level: &str) {
    let level = match log_level.to_uppercase().as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::ERROR,
    };

    tracing_subscriber::fmt().with_max_level(level).init();
}
