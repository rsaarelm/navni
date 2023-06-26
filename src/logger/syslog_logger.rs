use std::env;

use log::LevelFilter;

pub fn start(pname: &str) {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: pname.into(),
        pid: 0,
    };
    let Ok(logger) = syslog::unix(formatter) else {
        // Failed to connect to syslog.
        // Do we want to report an error here?
        // Going to just fail silently now.
        return;
    };

    let level = get_level().unwrap_or(LevelFilter::Warn);

    log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
        .map(|()| log::set_max_level(level))
        .unwrap();
}

fn get_level() -> Result<LevelFilter, ()> {
    // Nowhere near as fancy as env_logger's parsing, but it lets you set the
    // different logging levels.
    let var = env::var("RUST_LOG").map_err(|_| ())?;
    var.parse().map_err(|_| ())
}
