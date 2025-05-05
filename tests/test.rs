use std::{thread::sleep, time::Duration};

use qx_rs_err::err::*;
use qx_rs_md_env::md_env;
use qx_rs_md_log::md_log;
use log;

#[test]  
fn test() -> Result<()> {

    md_env::setup(Some("dev"))?;
    md_log::setup()?;

    for i in 1..100 {
        sleep(Duration::from_secs(1));
        log::info!("test-{i}");
    }

    Ok(())

}
