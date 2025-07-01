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
        variable::{ReadVariableInput, UpdateVariableInput},
    },
    printer::{Icon, Printer},
    util::{FlowletResult, clean_command, extract_json_path, launch_editor},
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

    pub async fn run(
        ctx: &impl WithContext,
        name: String,
        save_var: Option<String>,
        json_path: Option<String>,
    ) -> FlowletResult<()> {
        use crate::flowlet_db::models::variable::{CreateVariableInput, Variable};

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

        if command.cmd.is_empty() {
            return Err(Box::new(CliCommandError::EmptyCommand(command.name)));
        }

        let cleaned_command = clean_command(&command.cmd.clone());

        Printer::info(Icon::Rocket, "Running Command:", &command.name);

        // Capture output
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&cleaned_command)
            .output()
            .await
            .map_err(|e| {
                log::error!("Failed to run command: {:?}", e);
                CliCommandError::CommandExecutionFailed
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // 👇 Actually print the output
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }

        if !stderr.trim().is_empty() {
            eprintln!("{}", stderr);
        }

        if !output.status.success() {
            return Err(Box::new(CliCommandError::CommandExitedWithError(
                output
                    .status
                    .code()
                    .map_or("unknown".to_string(), |code| code.to_string()),
            )));
        }

        // If we want to save the result
        if let Some(var_name) = save_var {
            let exists = Variable::read(
                ctx.get(),
                ReadVariableInput {
                    query: Query::eq("name", var_name.clone()),
                },
            )
            .await?;

            let value_to_save = if let Some(path) = json_path {
                // Try to parse as JSON and extract value at path
                match serde_json::from_str::<serde_json::Value>(&stdout) {
                    Ok(json_value) => extract_json_path(&json_value, &path)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| {
                            Printer::warning(
                                Icon::Warning,
                                "Path Not Found",
                                "Falling back to full response.",
                            );
                            stdout.clone()
                        }),
                    Err(_) => {
                        Printer::warning(
                            Icon::Warning,
                            "Invalid JSON",
                            "Falling back to raw output.",
                        );
                        stdout.clone()
                    }
                }
            } else {
                stdout.clone()
            };

            match exists {
                Some(_) => {
                    Variable::update(
                        ctx.get(),
                        UpdateVariableInput {
                            name: var_name.clone(),
                            value: value_to_save.clone(),
                        },
                    )
                    .await?;
                }
                None => {
                    // Save to Variable
                    Variable::create(
                        ctx.get(),
                        CreateVariableInput {
                            name: var_name.clone(),
                            value: value_to_save.clone(),
                        },
                    )
                    .await?;
                }
            }

            Printer::success(
                Icon::Success,
                "Saved Variable",
                &format!("${} = {}", var_name, value_to_save),
            );
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
