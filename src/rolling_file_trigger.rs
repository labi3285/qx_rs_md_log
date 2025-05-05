use qx_rs_time::time;

use std::sync::RwLock;
use chrono::{DateTime, Utc};
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;

#[derive(Debug)]
pub struct RollingFileTrigger {
    last_date: RwLock<DateTime<Utc>>
}
impl RollingFileTrigger {
    pub fn new() -> Self {
        RollingFileTrigger { last_date: RwLock::new(time::now()) }
    }
}
impl Trigger for RollingFileTrigger {
    fn trigger(&self, _file: &log4rs::append::rolling_file::LogFile) -> anyhow::Result<bool> {
        let lock = self.last_date.read().unwrap();
        let now_date = time::now();
        if time::equal_day(now_date, *lock) {
            Ok(false)
        } else {
            drop(lock);
            let mut lock = self.last_date.write().unwrap();
            *lock = now_date;
            Ok(true)
        }
    }
    fn is_pre_process(&self) -> bool {
        true
    }
}