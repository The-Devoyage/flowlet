use deeb::Query;
use dialoguer::Confirm;
use thiserror::Error;

use crate::{
    flowlet_context::WithContext,
    flowlet_db::models::{
        self, Api,
        command::{
            CreateCommandInput, ListCommandInput, ReadCommandInput, RemoveCommandInput,
            UpdateCommandInput,
        },
    },
    printer::Printer,
    util::FlowletResult,
};

#[derive(Debug, Error)]
pub enum CliCommandError {
    #[error("Command not found.")]
    CommandNotFound,

    #[error("No command found for alias `{0}`.")]
    EmptyCommand(String),

    #[error("Command execution failed.")]
    CommandExecutionFailed,

    #[error("Command exited with error: {0}")]
    CommandExitedWithError(String),
}

pub struct Command;

impl Command {
    pub async fn save(ctx: &impl WithContext, name: String, cmd: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: deeb::Query::eq("name", name.clone()),
            },
        )
        .await?;

        if command.is_some() {
            models::command::Command::update(
                ctx.get(),
                UpdateCommandInput {
                    name: name.clone(),
                    cmd,
                },
            )
            .await?;
        } else {
            models::command::Command::create(
                ctx.get(),
                CreateCommandInput {
                    name: name.clone(),
                    cmd,
                },
            )
            .await?;
        }

        Printer::success(
            "Command",
            &format!("Saved your command. Run with `flowlet run {}`.", name),
        );
        Ok(())
    }

    pub async fn list(ctx: &impl WithContext, remote: bool) -> FlowletResult<()> {
        let commands = models::command::Command::list(
            ctx.get(),
            ListCommandInput {
                query: deeb::Query::All,
                remote,
            },
        )
        .await?;

        let rows: Vec<Vec<String>> = commands
            .into_iter()
            .map(|cmd| vec![cmd.name, cmd.cmd])
            .collect();

        Printer::success("Success", "Found commands!");
        Printer::table(vec!["Name", "Command"], rows);
        Ok(())
    }

    pub async fn run(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name),
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        // Make sure the command vector is not empty
        if command.cmd.is_empty() {
            return Err(Box::new(CliCommandError::EmptyCommand(command.name)));
        }

        Printer::info(&command.name, &command.cmd);

        let status = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&command.cmd) // run it as a shell string
            .spawn()
            .map_err(|e| {
                log::error!("Failed to spawn command: {:?}", e);
                CliCommandError::CommandExecutionFailed
            })?
            .wait()
            .await
            .map_err(|e| {
                log::error!("Failed to wait for command: {:?}", e);
                CliCommandError::CommandExecutionFailed
            })?;

        if !status.success() {
            return Err(Box::new(CliCommandError::CommandExitedWithError(
                status
                    .code()
                    .map_or("unknown".to_string(), |code| code.to_string()),
            )));
        }

        Ok(())
    }

    pub async fn show(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name),
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        Printer::info(&command.name, &command.cmd);

        Ok(())
    }

    pub async fn remove(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let confirm = Confirm::new()
            .with_prompt(format!("Are you sure you want to delete '{}'? [y/N]", name))
            .default(false)
            .interact()
            .unwrap();

        if !confirm {
            println!("Aborted.");
            return Ok(());
        }

        // Proceed with deletion
        models::command::Command::remove(ctx.get(), RemoveCommandInput { name: name.clone() })
            .await?;

        Printer::success(&"🗑️  Trashed", &format!("Command Removed: `{}`", name));

        Ok(())
    }
}
