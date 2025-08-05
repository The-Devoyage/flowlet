use deeb::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::{
    flowlet_context::FlowletContext,
    printer::{Icon, Printer},
    util::FlowletResult,
};

use super::Api;

#[derive(Collection, Deserialize, Serialize)]
pub struct Command {
    pub _id: ulid::Ulid,
    pub name: String,
    pub cmd: String,
    pub project: Option<String>
}

#[derive(Serialize)]
pub struct CreateCommandInput {
    pub name: String,
    pub cmd: String,
    pub project: Option<String>
}

#[derive(Serialize)]
pub struct UpdateCommandInput {
    pub name: String,
    pub cmd: String,
}

#[derive(Serialize)]
pub struct RemoveCommandInput {
    pub name: String,
}

#[derive(Serialize)]
pub struct ReadCommandInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Serialize)]
pub struct ListCommandInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Debug, Error)]
pub enum CommandApiError {
    #[error("Failed to save command.")]
    SaveCommandFailed,

    #[error("Failed to read command.")]
    ReadCommandFailed,

    #[error("No commands found.")]
    NoCommandsFound,

    #[error("Command not found.")]
    CommandNotFound,

    #[error("Failed to update remote.")]
    UpdateRemoteFailed,

    #[error("Failed to delete command.")]
    DeleteCommandFailed,
}

impl Api for Command {
    type CreateInput = CreateCommandInput;

    async fn create(
        flowlet_context: &FlowletContext,
        input: Self::CreateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        let command = Command::insert_one(
            deeb,
            Command {
                _id: ulid::Ulid::new(),
                name: input.name,
                cmd: input.cmd,
                project: input.project
            },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            CommandApiError::SaveCommandFailed
        })?;

        let _ = client
            .post::<Command, Command>("/insert-one/command", &command)
            .await
            .map_err(|_| {
                Printer::error(
                    Icon::Cloud,
                    "Save Command",
                    "Failed to save command to remote server.",
                )
            });

        Ok(command)
    }

    type UpdateInput = UpdateCommandInput;
    async fn update(
        flowlet_context: &FlowletContext,
        input: Self::UpdateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        let query = Query::eq("name", input.name.clone());

        let command = Command::update_one::<UpdateCommandInput>(
            deeb,
            query.clone(),
            UpdateCommandInput {
                name: input.name,
                cmd: input.cmd,
            },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            CommandApiError::SaveCommandFailed
        })?;

        if command.is_none() {
            return Err(Box::new(CommandApiError::CommandNotFound));
        }

        let command = command.unwrap();

        Printer::info(Icon::Local, "Success", "Command updated on local.");

        // Update Remote
        let remote_command = client
            .post::<_, Command>(
                "/update-one/command",
                &json!({"query": query.clone(), "document": command}),
            )
            .await?;

        if remote_command.data.is_none() {
            log::error!("Command data not found.");
            return Err(Box::new(CommandApiError::UpdateRemoteFailed));
        }

        Ok(command)
    }

    type ReadInput = ReadCommandInput;
    async fn read(
        flowlet_context: &FlowletContext,
        input: Self::ReadInput,
    ) -> FlowletResult<Option<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        if input.remote {
            Printer::info(Icon::Cloud, "Remote", "Fetching command...");
            let command = client
                .post::<_, Command>("/find-one/command", &json!({"query": input.query}))
                .await?;

            if command.data.is_none() {
                log::warn!("Command data not found.");
                return Ok(None);
            }

            return Ok(command.data);
        } else {
            let command = Command::find_one(deeb, input.query.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    CommandApiError::ReadCommandFailed
                })?;

            Ok(command)
        }
    }

    type ListInput = ListCommandInput;
    async fn list(
        flowlet_context: &FlowletContext,
        input: Self::ListInput,
    ) -> FlowletResult<Vec<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        if input.remote {
            Printer::info(Icon::Cloud, "Remote", "Fetching commands...");
            let commands = client
                .post::<_, Vec<Command>>("/find-many/command", &json!({"query": Query::All}))
                .await?;

            if commands.data.is_none() {
                log::error!("Commands data not found.");
                return Err(Box::new(CommandApiError::NoCommandsFound));
            }

            return Ok(commands.data.unwrap());
        }
        Printer::info(Icon::Local, "Local", "Fetching commands...");

        let commands = Command::find_many(deeb, input.query.clone(), None, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                CommandApiError::ReadCommandFailed
            })?;

        if commands.is_none() {
            return Err(Box::new(CommandApiError::NoCommandsFound));
        }

        Ok(commands.unwrap())
    }

    type RemoveInput = RemoveCommandInput;
    async fn remove(
        flowlet_context: &FlowletContext,
        input: Self::RemoveInput,
    ) -> FlowletResult<bool> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        Printer::warning(
            Icon::Warning,
            "Warning",
            &format!("Removing command: `{}`", input.name),
        );

        let query = Query::eq("name", input.name);

        let success = client
            .post::<_, bool>("/delete-one/command", &json!({"query": query}))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                e
            });

        // If command on remote is not found, the DB throws error
        if success.is_err() {
            log::error!("Failed to delete command from remote.");
            Printer::warning(Icon::Cloud, "Remote Failed", "Command on remote not found.");
        }

        let commands = Command::delete_one(deeb, query, None).await.map_err(|e| {
            log::error!("{:?}", e);
            CommandApiError::DeleteCommandFailed
        });

        // If command on remote is not found, the DB throws error
        if commands.is_err() {
            Printer::warning(Icon::Local, "Local Failed", "Command on local not found.");
            return Ok(false);
        }

        Ok(commands.unwrap().unwrap_or(false))
    }
}
