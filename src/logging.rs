use log::LevelFilter;
use std::io::Write;

pub fn setup_logger() {
    env_logger::Builder::from_default_env().format(|buf, record| {
        writeln!(buf, "[{}] {}", record.level(), record.args())
    }).filter_level(LevelFilter::Info).init();
}