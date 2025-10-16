use crate::runner::Runner;

pub const COMMAND_ID_SYNC: &str = "__SYNC_LOCAL_DATABASE";
pub const COMMAND_ID_SWITCH: &str = "__SWITCH_PROFILE";

impl Runner {
    pub fn is_command_sync(&self, query: &String) -> bool {
        !self.config.command_sync.is_empty() && query == &self.config.command_sync
    }

    pub fn is_command_switch_profile(&self, query: &String) -> bool {
        !self.config.command_switch_profile.is_empty()
            && query.starts_with(&self.config.command_switch_profile)
    }

    pub fn get_profile_from_command(&self, query: &String) -> String {
        query
            .replace(&self.config.command_switch_profile, "")
            .trim()
            .to_string()
    }
}
