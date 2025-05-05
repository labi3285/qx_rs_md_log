use qx_rs_time::time;

use std::fs;
use log4rs::append::rolling_file::policy::compound::roll::Roll;

#[derive(Debug)]
pub struct RollingFileRoller {
    time_zone: i32,
}
impl RollingFileRoller {
    pub fn new(time_zone: i32) -> Self {
        RollingFileRoller { time_zone }
    }
}
impl Roll for RollingFileRoller {
    fn roll(&self, file: &std::path::Path) -> anyhow::Result<()> {
        let date_str = time::format(time::now(), time::Pattern::Date, self.time_zone);
        let rename = format!("{}.log", date_str);
        let parent = file.parent().unwrap();
        fs::rename(file, parent.join(rename))?;
        Ok(())
    }
}