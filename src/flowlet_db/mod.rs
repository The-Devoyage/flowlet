use deeb::Deeb;
use dirs::home_dir;
use models::{auth::Auth, command::Command, user::User};
use thiserror::Error;

use crate::util::FlowletResult;

pub mod models;

pub struct FlowletDb {
    pub deeb: Deeb,
}

#[derive(Error, Debug)]
pub enum FlowletDbError {
    #[error("Failed to create DB Instance.")]
    InstanceCreationFailed,

    #[error("Failed to access home directory.")]
    HomeDirAccessDenied,
}

impl FlowletDb {
    pub async fn new() -> FlowletResult<Self> {
        let deeb = Deeb::new();

        // Init Models
        let auth = Auth::entity();
        let user = User::entity();
        let command = Command::entity();

        // Persist Dir
        let home = home_dir().ok_or(FlowletDbError::HomeDirAccessDenied)?;

        deeb.add_instance(
            "local",
            &format!("{}/.flowlet.json", home.to_str().unwrap()),
            vec![auth, user, command],
        )
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            FlowletDbError::InstanceCreationFailed
        })?;

        Ok(FlowletDb { deeb })
    }
}
