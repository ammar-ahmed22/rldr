use anyhow::{anyhow, Result};
use colored::*;
use log::{Level, LevelFilter, Metadata, Record};

pub struct Logger;

impl Logger {
    pub fn colored(level: Level, str: String) -> String {
        match level {
            Level::Info => str.cyan().to_string(),
            Level::Error => str.red().to_string(),
            Level::Warn => str.yellow().to_string(),
            _ => String::new(),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{}",
                Self::colored(record.level(), format!("[rldr] {}", record.args()))
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_logger() -> Result<(), anyhow::Error> {
    log::set_logger(&LOGGER).map_err(|err| anyhow!("{}", err))?;
    log::set_max_level(LevelFilter::Info);
    return Ok(());
}
