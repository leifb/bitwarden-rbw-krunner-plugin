use log::warn;
use std::process::Command;

/// Searches bitwarden entries using rbw
pub fn search(term: &str) -> Result<Vec<String>, String> {
    let mut call = Command::new("rbw");
    call.arg("search").arg(term.to_string());

    let result = match call.output() {
        Ok(output) => output,
        Err(e) => {
            warn!("Error running rbw: {}", e);
            return Err(e.to_string());
        }
    };

    if !result.status.success() {
        warn!(
            "Error running rbw: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        return Err(String::from_utf8_lossy(&result.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&result.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect())
}

/// Copies the password of an entry to the clipboard using rbw
pub fn copy(entry: String) -> Result<(), String> {
    let mut call = Command::new("rbw");
    call.arg("get").arg("-c").arg(entry);

    if let Err(e) = call.output() {
        warn!("Error running rbw: {}", e);
        return Err(e.to_string());
    };

    Ok(())
}

pub fn get_full_info(entry: String) -> Result<String, String> {
    let mut call = Command::new("rbw");
    call.arg("get").arg("--full").arg(entry);

    let result = match call.output() {
        Ok(output) => output,
        Err(e) => {
            warn!("Error running rbw: {}", e);
            return Err(e.to_string());
        }
    };

    if !result.status.success() {
        warn!(
            "Error running rbw: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        return Err(String::from_utf8_lossy(&result.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&result.stdout).to_string())
}

/// Manually triggers a sync in rbw.
pub fn sync() -> Result<(), String> {
    let mut call = Command::new("rbw");
    call.arg("sync");

    if let Err(e) = call.output() {
        warn!("Error running rbw: {}", e);
        return Err(e.to_string());
    };

    Ok(())
}
