use deeb::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::{flowlet_context::FlowletContext, printer::Printer, util::FlowletResult};

use super::Api;

#[derive(Collection, Deserialize, Serialize)]
pub struct Command {
    pub _id: ulid::Ulid,
    pub name: String,
    pub cmd: Vec<String>,
}

#[derive(Serialize)]
pub struct CreateCommandInput {
    pub name: String,
    pub cmd: Vec<String>,
}

#[derive(Serialize)]
pub struct UpdateCommandInput {
    pub name: String,
    pub cmd: Vec<String>,
}

#[derive(Debug, Error)]
pub enum CommandApiError {
    #[error("Failed to save command.")]
    SaveCommandFailed,

    #[error("Failed to read command.")]
    ReadCommandFailed,
}

#[derive(Serialize)]
pub struct ReadCommandInput {
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
            return Err(Box::new(CommandApiError::SaveCommandFailed));
        }

        let command = command.unwrap();

        let _ = client
            .post::<serde_json::Value, Command>(
                "/update-one/command",
                &json!({"document": command, "query": query}),
            )
            .await
            .map_err(|_| {
                Printer::error("Update Command", "Failed to sync command to remote server.")
            });

        Ok(command)
    }

    type ReadInput = ReadCommandInput;

    async fn read(
        flowlet_context: &FlowletContext,
        input: Self::ReadInput,
    ) -> FlowletResult<Option<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;

        let command = Command::find_one(deeb, input.query.clone(), None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                CommandApiError::ReadCommandFailed
            })?;

        if command.is_some() {
            return Ok(command);
        }

        let command = client
            .post::<serde_json::Value, Command>("/find-one/command", &json!({"query": input.query}))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                CommandApiError::ReadCommandFailed
            })?;

        Ok(command.data)
    }
}
