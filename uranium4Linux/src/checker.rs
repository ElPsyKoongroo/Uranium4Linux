use chrono::prelude::Local;
use once_cell::sync::Lazy;
use simplelog::*;
use std::fs::File;
static _LOGGER_INIT: Lazy<()> = Lazy::new(|| {
    let log_file_name = format!("log_{}.txt", Local::now().format("%H-%M-%S_%d-%m-%Y"));
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file_name).unwrap(),
        ),
    ])
    .unwrap();
    ()
});
