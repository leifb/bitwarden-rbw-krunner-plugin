use log::warn;
use std::process::Command;

#[derive(Default)]
pub struct RbwProfile {
    pub name: String,
}

impl RbwProfile {
    /// Searches bitwarden entries using rbw
    pub fn search(&self, term: &str) -> Result<Vec<String>, String> {
        let mut call = Command::new("rbw");
        call.env("RBW_PROFILE", &self.name)
            .arg("search")
            .arg(term.to_string());

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
    pub fn copy(&self, entry: String) -> Result<(), String> {
        let mut call = Command::new("rbw");
        call.env("RBW_PROFILE", &self.name)
            .arg("get")
            .arg("-c")
            .arg(entry);

        if let Err(e) = call.output() {
            warn!("Error running rbw: {}", e);
            return Err(e.to_string());
        };

        Ok(())
    }

    pub fn get_full_info(&self, entry: &str) -> Result<String, String> {
        let mut call = Command::new("rbw");
        call.env("RBW_PROFILE", &self.name)
            .arg("get")
            .arg("--full")
            .arg(entry);

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
    pub fn sync(&self) -> Result<(), String> {
        let mut call = Command::new("rbw");
        call.env("RBW_PROFILE", &self.name).arg("sync");

        if let Err(e) = call.output() {
            warn!("Error running rbw: {}", e);
            return Err(e.to_string());
        };

        Ok(())
    }
}
