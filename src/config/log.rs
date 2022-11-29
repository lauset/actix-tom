use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use fast_log::Config;
use log::LevelFilter;

pub fn init_logger() {
    fast_log::init(
        Config::new()
            .console()
            .file_split(
                "logs/",
                LogSize::MB(1),
                RollingType::All,
                LogPacker {},
            )
            .level(LevelFilter::Info),
    )
    .unwrap();
}
