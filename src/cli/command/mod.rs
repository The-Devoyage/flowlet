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
    printer::{Icon, Printer},
    util::{FlowletResult, clean_command, launch_editor},
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
                remote: false,
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
            Icon::Success,
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

        Printer::success(Icon::Success, "Success", "Found commands!");
        Printer::table(vec!["Name", "Command"], rows);
        Ok(())
    }

    pub async fn run(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name),
                remote: false,
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

        let cleaned_command = clean_command(&command.cmd.clone());

        // Print rocket icon with info label
        Printer::info(Icon::Rocket, "Running Command:", &command.name);
        Printer::info(Icon::Rocket, "", &cleaned_command);

        // Prepare headers and rows for your Printer::table
        let headers = vec!["Name", "Command"];

        let rows = vec![vec![command.name.clone(), command.cmd.clone()]];

        // Use your existing Printer::table function
        Printer::table(headers, rows);
        let status = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&cleaned_command) // run it as a shell string
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
                remote: false,
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        Printer::info(Icon::Rocket, "Show Command", &command.name);

        let cleaned = clean_command(&command.cmd);
        let cleaned_lines = cleaned.split('\n').collect::<Vec<&str>>();

        Printer::multi_line_info("To run manually:", cleaned_lines);

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

        Printer::success(
            Icon::Trash,
            "Trashed",
            &format!("Command Removed: `{}`", name),
        );

        Ok(())
    }

    pub async fn edit(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name.clone()),
                remote: false,
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        let text = launch_editor(&command.cmd)?;

        Self::save(ctx, name, text).await?;

        Printer::success(Icon::Success, "Saved", "Command has been updated.");

        Ok(())
    }

    pub async fn push(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        // Get the local command to push
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name.clone()),
                remote: false,
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        let remote = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name.clone()),
                remote: true,
            },
        )
        .await?;

        if remote.is_some() {
            models::command::Command::update(
                ctx.get(),
                UpdateCommandInput {
                    name: name.clone(),
                    cmd: command.cmd,
                },
            )
            .await?;
        } else {
            models::command::Command::create(
                ctx.get(),
                CreateCommandInput {
                    name: name.clone(),
                    cmd: command.cmd,
                },
            )
            .await?;
        }

        Printer::success(Icon::Success, "Saved", "Pushed command to remote.");

        Ok(())
    }

    pub async fn pull(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let command = models::command::Command::read(
            ctx.get(),
            ReadCommandInput {
                query: Query::eq("name", name.clone()),
                remote: true,
            },
        )
        .await?;

        let command = match command {
            Some(c) => c,
            None => return Err(Box::new(CliCommandError::CommandNotFound)),
        };

        Self::save(ctx, name, command.cmd).await?;

        Printer::success(Icon::Success, "Saved", "Pushed command to remote.");

        Ok(())
    }
}
