//! Log handling
//!
//! Handles all kinds of logs generated by either iota
//! wallet or iota client or the sdk itself
//!
//!

use fern_logger::{LoggerConfig, LoggerOutputConfigBuilder, logger_init};
use log::warn;

/// Initializes the logger
#[allow(clippy::result_large_err)]
pub fn init_logger(level: log::LevelFilter, file: &str) -> crate::Result<()> {
    let config = LoggerConfig::build()
        .with_output(LoggerOutputConfigBuilder::new().level_filter(level).name(file))
        .finish();
    match logger_init(config) {
        Err(fern_logger::Error::InitializationFailed) => {
            warn!("Attempt to initialize logger failed, is logger already initialized?");
            Ok(())
        }
        other => Ok(other.map_err(crate::Error::LoggerInit)?),
    }
}
