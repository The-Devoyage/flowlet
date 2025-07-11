use dialoguer::{Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use crate::{
    flowlet_context::WithContext,
    flowlet_db::models::{
        self, Api,
        project::{CreateProjectInput, ListProjectInput, Project, RemoveProjectInput},
    },
    printer::{Icon, Printer},
    util::FlowletResult,
};

#[derive(Debug, Error)]
pub enum CliProjectError {
    #[error("Project not found.")]
    NotFound,

    #[error("Failed to create project.")]
    CreateFailed,

    #[error("Failed to delete project.")]
    DeleteFailed,
}

#[derive(Serialize)]
struct FlowletConfig {
    project: ProjectConfig,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    description: Option<String>,
    environment: String,
}

pub struct ProjectCli;

impl ProjectCli {
    pub async fn new(ctx: &impl WithContext) -> FlowletResult<()> {
        let name: String = Input::new()
            .with_prompt("Enter a name for your project")
            .interact_text()?;

        let description: String = Input::<String>::new()
            .with_prompt("Enter a description (optional)")
            .allow_empty(true)
            .interact_text()?
            .trim()
            .to_owned();

        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        let environments = vec!["local", "dev", "staging", "prod"];
        let selected = Select::new()
            .with_prompt("Select current environment")
            .items(&environments)
            .default(0)
            .interact()?;

        let environment = environments[selected].to_string();

        let created = Project::create(
            ctx.get(),
            CreateProjectInput {
                name: name.clone(),
                description: description.clone(),
            },
        )
        .await?;

        let config = FlowletConfig {
            project: ProjectConfig {
                name: name.clone(),
                description,
                environment,
            },
        };

        // Serialize to TOML
        let rc_contents = toml::to_string_pretty(&config)?;
        let rc_path = PathBuf::from("flowlet.toml");
        fs::write(&rc_path, rc_contents)?;
        Printer::success(
            Icon::Success,
            "Project",
            &format!(
                "Created project `{}` and wrote `flowlet.toml`.",
                created.name
            ),
        );

        Ok(())
    }

    pub async fn remove(ctx: &impl WithContext, name: String) -> FlowletResult<()> {
        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete the project `{}`?",
                name
            ))
            .default(false)
            .interact()?;

        if !confirm {
            Printer::info(Icon::Warning, "Aborted", "Project deletion cancelled.");
            return Ok(());
        }

        Project::remove(ctx.get(), RemoveProjectInput { name: name.clone() }).await?;

        Printer::success(
            Icon::Trash,
            "Project",
            &format!("Deleted project `{}`.", name),
        );
        Ok(())
    }

    pub async fn list(ctx: &impl WithContext) -> FlowletResult<()> {
        let projects = models::project::Project::list(
            ctx.get(),
            ListProjectInput {
                query: deeb::Query::All,
                remote: false,
            },
        )
        .await?;

        let rows: Vec<Vec<String>> = projects
            .iter()
            .map(|p| {
                vec![
                    p.name.clone(),
                    p.description.clone().unwrap_or_else(|| "-".to_string()),
                ]
            })
            .collect();

        Printer::success(Icon::Project, "Projects", "Found your projects!");
        Printer::table(vec!["Name", "Description"], rows);

        Ok(())
    }
}
