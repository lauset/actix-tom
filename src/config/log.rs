use actix_settings::{Mode, Settings};
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use fast_log::Config;
use log::LevelFilter;

pub fn init_fast_logger() {
    fast_log::init(
        Config::new()
            .console()
            .file_split("logs/", LogSize::MB(1), RollingType::All, LogPacker {})
            .level(LevelFilter::Info),
    )
    .unwrap();
}

pub fn init_logger(settings: &Settings) {
    if !settings.actix.enable_log {
        return;
    }

    std::env::set_var(
        "RUST_LOG",
        match settings.actix.mode {
            Mode::Development => "debug",
            Mode::Production => "actix_tom=info",
        },
    );

    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
}
