mod commands;
mod config;
mod notify;
mod profile_discovery;
mod rbw;
mod runner;

use crate::config::Config;
use crate::profile_discovery::get_profiles;
use crate::rbw::RbwProfile;
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
    let runner = Runner {
        known_profiles: get_profiles(&config),
        current_profile: RbwProfile{ name: config.initial_profile.clone() },
        config,
    };
    runner.start(DBUS_NAME, DBUS_PATH)?;
    Ok(())
}
