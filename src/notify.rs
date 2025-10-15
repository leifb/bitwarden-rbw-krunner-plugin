use crate::rbw;
use crate::runner::Runner;
use notify_rust::Notification;

impl Runner {
    /// Gets the full info of a bitwarden entry and shows it as a notification.
    pub fn show_entry_info(&self, term: String) -> Result<(), String> {
        let info = rbw::get_full_info(term.clone())?;
        let info = self.remove_password(info);

        Notification::new()
            .summary(&format!("Bitwarden Entry '{}'", term))
            .body(&info)
            .icon("lock")
            .show()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Removes the password from an entries' info if the config is set to do so.
    fn remove_password(&self, info: String) -> String {
        if self.config.show_password {
            return info;
        }

        info.lines().skip(1).collect::<Vec<&str>>().join("\n")
    }
}
