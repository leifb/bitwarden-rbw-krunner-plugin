use crate::config::Config;
use log::warn;
use std::path::Path;
use std::{env, fs, io};

pub fn get_profiles(config: &Config) -> Vec<String> {
    let mut profiles = vec!["".to_owned()];
    if !config.discover_profiles {
        return profiles;
    }

    let Ok(home) = env::var("HOME") else {
        warn!("Failed to search for rbw profiles: Home unknown");
        return profiles;
    };

    let home = Path::new(&home);

    let dirs = match find_rbw_dirs_with_config(home) {
        Ok(dirs) => dirs,
        Err(e) => {
            warn!("Failed to search for rbw profiles: {}", e);
            return profiles;
        }
    };

    let mut additional_profiles: Vec<String> =
        dirs.into_iter().map(|x| x[4..].to_owned()).collect();

    profiles.append(&mut additional_profiles);
    profiles
}

fn find_rbw_dirs_with_config(home: &Path) -> io::Result<Vec<String>> {
    let mut dirs = Vec::new();
    let config_dir = home.join(".config/");
    for child in config_dir.read_dir()? {
        let child = child?;
        let file_type = child.file_type()?;
        if !file_type.is_dir() {
            continue;
        }

        let file_name = child.file_name().to_str().unwrap_or("").to_owned();
        if !file_name.starts_with("rbw-") {
            continue;
        }

        let mut children = fs::read_dir(child.path())?;
        let has_config = children
            .any(|c| c.is_ok_and(|x| x.file_name().to_str().unwrap_or("") == "config.json"));
        if !has_config {
            continue;
        }

        dirs.push(file_name);
    }

    Ok(dirs)
}
