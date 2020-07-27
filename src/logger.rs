#![deny(unsafe_code)]

use anyhow::{anyhow, Result};
use simplelog::SimpleLogger;

pub async fn setup_logger(minimum_logging_level: u8) -> Result<()> {
    let log_level;
    match minimum_logging_level {
        0 => {
            log_level = simplelog::LevelFilter::Trace;
        }
        1 => {
            log_level = simplelog::LevelFilter::Debug;
        }
        2 => {
            log_level = simplelog::LevelFilter::Info;
        }
        3 => {
            log_level = simplelog::LevelFilter::Warn;
        }
        4 => {
            log_level = simplelog::LevelFilter::Error;
        }
        _ => {
            return Err(anyhow!(
                "Unknown/unsupported minimum_logging_level {:?}",
                minimum_logging_level
            ));
        }
    }
    let _ = SimpleLogger::init(log_level, simplelog::Config::default());
    Ok(())
}
