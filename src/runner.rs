use crate::commands::{COMMAND_ID_SWITCH, COMMAND_ID_SYNC};
use crate::config::Config;
use crate::rbw::RbwProfile;
use krunner::Match;

const TITLE_SYNC: &str = "Sync database";

#[derive(krunner::Action)]
pub enum Action {
    #[action(
        id = "show",
        title = "Show entry in a notification",
        icon = "structure"
    )]
    Show,
}

pub struct Runner {
    pub config: Config,
    pub known_profiles: Vec<String>,
    pub current_profile: RbwProfile,
}

impl krunner::Runner for Runner {
    type Action = Action;
    type Err = String;

    fn matches(&mut self, query: String) -> Result<Vec<Match<Self::Action>>, Self::Err> {
        // Handle special command to sync
        if self.is_command_sync(&query) {
            return Ok(vec![Match {
                id: COMMAND_ID_SYNC.to_owned(),
                title: TITLE_SYNC.to_owned(),
                icon: "view-refresh".to_owned().into(),
                ..Match::default()
            }]);
        }

        if self.is_command_switch_profile(&query) {
            return Ok(self.get_readable_known_profiles(query));
        }

        if !query.starts_with(&self.config.prefix) {
            return Ok(vec![]);
        }

        let term = &query[self.config.prefix.len()..];
        if term.len() < self.config.min_length {
            return Ok(vec![]);
        }

        let results = self.current_profile.search(term)?;
        let matches = results
            .into_iter()
            .map(|r| Match {
                id: r.clone(),
                title: r,
                icon: "password-copy".to_owned().into(),
                ..Match::default()
            })
            .collect();
        Ok(matches)
    }

    fn run(&mut self, match_id: String, action: Option<Self::Action>) -> Result<(), Self::Err> {
        if match_id == COMMAND_ID_SYNC {
            return self.current_profile.sync();
        }

        if match_id.starts_with(COMMAND_ID_SWITCH) {
            return self.switch_profile(match_id);
        }

        if action.is_some() {
            self.show_entry_info(match_id)
        } else {
            self.current_profile.copy(match_id)
        }
    }
}

impl Runner {
    fn switch_profile(&mut self, command: String) -> Result<(), String> {
        let Some((_, profile)) = command.split_once(" ") else {
            return Err("Invalid switch profile command".to_owned());
        };
        self.current_profile = RbwProfile {
            name: profile.to_owned(),
        };
        Ok(())
    }

    fn get_readable_known_profiles(&self, query: String) -> Vec<Match<Action>> {
        let selection = self.get_profile_from_command(&query);
        let mut profiles = self.known_profiles.clone();
        if let Some(position) = profiles.iter().position(|x| x == &selection) {
            profiles.swap(position, 0);
        } else {
            profiles.insert(0, selection);
        }

        profiles
            .iter()
            .enumerate()
            .map(|(i, profile)| self.create_switch_profile_match(i, profile))
            .collect()
    }

    fn get_switch_profile_text(&self, profile: &String) -> String {
        if profile.is_empty() {
            "Use default profile".to_owned()
        } else {
            format!("Use '{}' profile", profile)
        }
    }

    fn create_switch_profile_match(&self, i: usize, profile: &String) -> Match<Action> {
        let is_current = profile == &self.current_profile.name;
        let subtitle = if is_current {
            Some("current".to_string())
        } else {
            None
        };
        let relevance = if is_current { 0.0 } else { 1.0 / i as f64 };
        Match {
            id: format!("{} {}", COMMAND_ID_SWITCH, profile),
            title: self.get_switch_profile_text(profile),
            icon: "user".to_owned().into(),
            relevance,
            subtitle,
            ..Match::default()
        }
    }
}
