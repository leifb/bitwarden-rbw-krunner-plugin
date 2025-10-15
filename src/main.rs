mod config;
mod notify;
mod rbw;
mod runner;

use crate::config::Config;
use crate::runner::Runner;
use krunner::RunnerExt;
use log::info;
use std::env;

const DBUS_NAME: &str = "de.leifb.BitwardenRbwKrunner";
const DBUS_PATH: &str = "/runner";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!(
        "Starting {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let config = Config::load()?;
    let runner = Runner { config };
    runner.start(DBUS_NAME, DBUS_PATH)?;
    Ok(())
}
