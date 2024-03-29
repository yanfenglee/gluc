use std::time::Duration;

use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::ZipPacker},
};

pub fn init_log() {
    fast_log::init_split_log(
        "target/logs/",
        LogSize::MB(1),
        RollingType::KeepTime(Duration::from_secs(3600 * 24)),
        log::Level::Info,
        None,
        Box::new(ZipPacker {}),
        true,
    );
}
