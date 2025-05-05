use qx_rs_err::err::*;
use qx_rs_md_env::md_env;
use qx_rs_str::path;

use crate::rolling_file_trigger;
use crate::rolling_file_roller;

use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{policy::compound::CompoundPolicy, RollingFileAppender},
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

pub fn setup() -> Result<()> {
    let log_pattern = "{d(%H:%M:%S%.3f)}[{M}]<{l}>:{m}{n}";
    let config = if cfg!(debug_assertions) {
        let level = if let Some(v) = md_env::str("log.level") {
            match v.to_lowercase().as_str() {
                "trace" => { log::LevelFilter::Trace },
                "debug" => { log::LevelFilter::Debug },
                "info" => { log::LevelFilter::Info },
                "warn" => { log::LevelFilter::Warn },
                "error" => { log::LevelFilter::Error },
                "off" => { log::LevelFilter::Off },
                _ => { log::LevelFilter::Trace }
            }
        } else {
            log::LevelFilter::Trace
        };
        let appender = _console_appender(log_pattern)?;
        let root = Root::builder().appenders(["stdout"]).build(level);
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(appender)))
            .build(root)
            .map_err(|err| {
                Error::error(Box::new(err))
            })?
    } else {
        let level = if let Some(v) = md_env::str("log.level") {
            match v.to_lowercase().as_str() {
                "trace" => { log::LevelFilter::Trace },
                "debug" => { log::LevelFilter::Debug },
                "info" => { log::LevelFilter::Info },
                "warn" => { log::LevelFilter::Warn },
                "error" => { log::LevelFilter::Error },
                "off" => { log::LevelFilter::Off },
                _ => { log::LevelFilter::Info }
            }
        } else {
            log::LevelFilter::Info
        };
        let appender = _rolling_file_appender(log_pattern)?;
        let root = Root::builder().appenders(["rolling_file"]).build(level);
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(appender)))
            .build(root)
            .map_err(|err| {
                Error::error(Box::new(err))
            })?
    };
    log4rs::init_config(config).map_err(|err| {
        Error::error(Box::new(err))
    })?;
    Ok(())
}

fn _console_appender(log_pattern: &str) -> Result<ConsoleAppender> {
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .target(Target::Stdout)
        .build();
    Ok(appender)
}
fn _rolling_file_appender(log_pattern: &str) -> Result<RollingFileAppender> {
    let log_path = md_env::str("log.path").unwrap_or("logs".into());
    let time_zone = md_env::val("time.zone")?.unwrap_or(0);
    let trigger = rolling_file_trigger::RollingFileTrigger::new();
    let roller = rolling_file_roller::RollingFileRoller::new(time_zone);
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build(path::combine(&log_path, "run.log"), Box::new(policy))
        .map_err(|err| {
            Error::error(Box::new(err))
        })?;
    Ok(appender)
}
