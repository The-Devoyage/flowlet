use deeb::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{flowlet_context::FlowletContext, printer::Printer, util::FlowletResult};

use super::Api;

#[derive(Collection, Deserialize, Serialize)]
pub struct Command {
    pub _id: ulid::Ulid,
    pub name: String,
    pub cmd: String,
}

#[derive(Serialize)]
pub struct CreateCommandInput {
    pub name: String,
    pub cmd: String,
}

#[derive(Serialize)]
pub struct UpdateCommandInput {
    pub name: String,
    pub cmd: String,
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
}

#[derive(Serialize)]
pub struct ReadCommandInput {
    pub query: Query,
}

#[derive(Serialize)]
pub struct ListCommandInput {
    pub query: Query,
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
                Printer::error("Save Command", "Failed to save command to remote server.")
            });

        Ok(command)
    }

    type UpdateInput = UpdateCommandInput;
    async fn update(
        flowlet_context: &FlowletContext,
        input: Self::UpdateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;

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

        Ok(command)
    }

    type ReadInput = ReadCommandInput;

    async fn read(
        flowlet_context: &FlowletContext,
        input: Self::ReadInput,
    ) -> FlowletResult<Option<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let command = Command::find_one(deeb, input.query.clone(), None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                CommandApiError::ReadCommandFailed
            })?;

        Ok(command)
    }

    type ListInput = ListCommandInput;
    async fn list(
        flowlet_context: &FlowletContext,
        input: Self::ListInput,
    ) -> FlowletResult<Vec<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;

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
}
