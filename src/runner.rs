use crate::config::Config;
use crate::rbw;
use krunner::Match;

const ID_SYNC: &str = "__SYNC_LOCAL_DATABASE";
const TITLE_SYNC: &str = "Sync local Bitwarden database";

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
}

impl krunner::Runner for Runner {
    type Action = Action;
    type Err = String;

    fn matches(&mut self, query: String) -> Result<Vec<Match<Self::Action>>, Self::Err> {
        // Handle special command to sync
        if query == self.config.sync_command && !self.config.sync_command.is_empty() {
            return Ok(vec![Match {
                id: ID_SYNC.to_owned(),
                title: TITLE_SYNC.to_owned(),
                icon: "view-refresh".to_owned().into(),
                ..Match::default()
            }]);
        }

        if !query.starts_with(&self.config.prefix) {
            return Ok(vec![]);
        }

        let term = &query[self.config.prefix.len()..];
        if term.len() < self.config.min_length {
            return Ok(vec![]);
        }

        let results = rbw::search(term)?;
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
        if match_id == ID_SYNC {
            return rbw::sync();
        }

        if action.is_some() {
            self.show_entry_info(match_id)
        } else {
            rbw::copy(match_id)
        }
    }
}
