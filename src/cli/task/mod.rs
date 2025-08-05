use deeb::Query;
use dialoguer::{Confirm, Input, Select};
use std::str::FromStr;
use thiserror::Error;

use crate::{
    flowlet_context::WithContext,
    flowlet_db::models::{
        self, Api,
        task::{
            CreateTaskInput, ListTaskInput, Milestone, ReadTaskInput, RemoveTaskInput, Task,
            TaskStatus, UpdateTaskInput,
        },
    },
    printer::{Icon, Printer},
    util::{FlowletResult, find_project_config, request_date_input, truncate_with_ellipsis},
};

#[derive(Debug, Error)]
pub enum CliTaskError {
    #[error("Task not found.")]
    NotFound,

    #[error("Failed to create task.")]
    CreateFailed,

    #[error("Failed to delete task.")]
    DeleteFailed,
}

pub struct TaskCli;

impl TaskCli {
    pub async fn new(ctx: &impl WithContext) -> FlowletResult<()> {
        let title: String = Input::new()
            .with_prompt("Enter a title for the task")
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

        let due_date = request_date_input("Enter due date (YYYY-MM-DD, optional)", true)?;

        let tags_input: String = Input::new()
            .with_prompt("Enter tags (comma-separated, optional)")
            .allow_empty(true)
            .interact_text()?;

        let tags: Vec<String> = tags_input
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();

        let mut milestones = Vec::new();
        loop {
            let add_milestone = Confirm::new()
                .with_prompt("Add a milestone?")
                .default(false)
                .interact()?;

            if !add_milestone {
                break;
            }

            let milestone_name: String =
                Input::new().with_prompt("Milestone name").interact_text()?;

            let milestone_desc: String = Input::new()
                .with_prompt("Milestone description")
                .interact_text()?;

            let milestone_due =
                request_date_input("Milestone due date (YYYY-MM-DD, optional)", true)?;

            milestones.push(Milestone {
                name: milestone_name,
                description: milestone_desc,
                due_date: milestone_due,
            });
        }

        let project = find_project_config().ok().flatten();

        let created = Task::create(
            ctx.get(),
            CreateTaskInput {
                title,
                description,
                project,
                due_date,
                tags,
                milestones,
            },
        )
        .await?;

        Printer::success(
            Icon::Success,
            "Task",
            &format!("Created task `{}`", created.title),
        );
        Ok(())
    }

    pub async fn edit(ctx: &impl WithContext, _id: String) -> FlowletResult<()> {
        let task = Task::read(
            ctx.get(),
            ReadTaskInput {
                query: Query::eq("_id", _id.clone()),
                remote: false,
            },
        )
        .await?;

        let task = match task {
            Some(t) => t,
            None => {
                Printer::error(
                    Icon::Error,
                    "Task",
                    &format!("Task with _id `{}` not found", _id),
                );
                return Ok(());
            }
        };

        Printer::info(Icon::Task, "Editing Task", &task.title);

        let new_title: String = Input::new()
            .with_prompt("Title")
            .default(task.title.clone())
            .interact_text()?;

        let new_description: String = Input::new()
            .with_prompt("Description (optional)")
            .default(task.description.clone().unwrap_or_default())
            .allow_empty(true)
            .interact_text()?;

        let due_date = request_date_input("Due Date (YYYY-MM-DD, optional)", true)?;

        let tags_input: String = Input::new()
            .with_prompt("Tags (comma-separated)")
            .default(task.tags.join(", "))
            .allow_empty(true)
            .interact_text()?;

        let tags: Vec<String> = tags_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Status update
        let statuses = vec!["Todo", "In Progress", "Done"];
        let current_index = match task.status {
            TaskStatus::Todo => 0,
            TaskStatus::InProgress => 1,
            TaskStatus::Done => 2,
        };

        let status_index = Select::new()
            .with_prompt("Status")
            .items(&statuses)
            .default(current_index)
            .interact()?;

        let status = match status_index {
            0 => TaskStatus::Todo,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::Done,
            _ => unreachable!(),
        };

        // Edit milestones
        let mut milestones = task.milestones.clone();

        if Confirm::new()
            .with_prompt("Edit milestones?")
            .default(false)
            .interact()?
        {
            for i in 0..milestones.len() {
                let ms = &mut milestones[i];
                let name: String = Input::new()
                    .with_prompt(&format!("Milestone #{} name", i + 1))
                    .default(ms.name.clone())
                    .interact_text()?;

                let desc: String = Input::new()
                    .with_prompt("Description")
                    .default(ms.description.clone())
                    .interact_text()?;

                let due_input: String = Input::new()
                    .with_prompt("Due date (YYYY-MM-DD, optional)")
                    .default(ms.due_date.map(|d| d.to_string()).unwrap_or_default())
                    .allow_empty(true)
                    .interact_text()?;

                let due_date = if due_input.trim().is_empty() {
                    None
                } else {
                    Some(chrono::NaiveDate::from_str(&due_input)?)
                };

                ms.name = name;
                ms.description = desc;
                ms.due_date = due_date;
            }

            // Add more milestones
            loop {
                let add = Confirm::new()
                    .with_prompt("Add another milestone?")
                    .default(false)
                    .interact()?;

                if !add {
                    break;
                }

                let name: String = Input::new().with_prompt("Milestone name").interact_text()?;
                let desc: String = Input::new().with_prompt("Description").interact_text()?;
                let due_date = request_date_input("Due date (YYYY-MM-DD, optional)", true)?;
                milestones.push(Milestone {
                    name,
                    description: desc,
                    due_date,
                });
            }
        }

        let updated = Task::update(
            ctx.get(),
            UpdateTaskInput {
                title: new_title.clone(),
                description: if new_description.trim().is_empty() {
                    None
                } else {
                    Some(new_description)
                },
                due_date,
                tags: Some(tags),
                milestones: Some(milestones),
                status: Some(status),
            },
        )
        .await?;

        Printer::success(
            Icon::Success,
            "Task",
            &format!("Updated task `{}`", updated.title),
        );

        Ok(())
    }

    pub async fn list(ctx: &impl WithContext, remote: bool, global: bool) -> FlowletResult<()> {
        let project = find_project_config().ok().flatten();

        let mut query = Query::All;

        if let Some(project) = project {
            if !global {
                Printer::info(Icon::Project, "Project Selected:", project.as_str());
                query = Query::eq("project", project);
            }
        }

        let tasks = models::task::Task::list(ctx.get(), ListTaskInput { query, remote }).await?;

        let rows: Vec<Vec<String>> = tasks
            .iter()
            .map(|t| {
                vec![
                    t._id.to_string(),
                    truncate_with_ellipsis(&t.title, 40),
                    t.status.to_string(),
                    t.project.clone().unwrap_or_else(|| "-".to_string()),
                    t.due_date
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| "-".to_string()),
                    t.milestones.len().to_string(),
                ]
            })
            .collect();

        Printer::success(Icon::Project, "Tasks", "Found your tasks!");
        Printer::table(
            vec!["_id", "Title", "Status", "Project", "Due", "Milestones"],
            rows,
        );

        Ok(())
    }

    pub async fn show(ctx: &impl WithContext, _id: String) -> FlowletResult<()> {
        let task = Task::read(
            ctx.get(),
            ReadTaskInput {
                query: Query::eq("_id", _id.clone()),
                remote: false,
            },
        )
        .await?;

        let task = match task {
            Some(t) => t,
            None => {
                Printer::error(
                    Icon::Error,
                    "Task",
                    &format!("No task found with _id `{}`", _id),
                );
                return Ok(());
            }
        };

        Printer::info(Icon::Task, "Task Details", &task.title);

        let fields = vec![
            ("Title", task.title.clone()),
            (
                "Project",
                task.project.clone().unwrap_or_else(|| "-".into()),
            ),
            (
                "Description",
                task.description.clone().unwrap_or_else(|| "-".into()),
            ),
            (
                "Due Date",
                task.due_date.map_or("-".into(), |d| d.to_string()),
            ),
            (
                "Tags",
                if task.tags.is_empty() {
                    "-".into()
                } else {
                    task.tags.join(", ")
                },
            ),
            ("Status", task.status.to_string()),
        ];

        Printer::block_kv("Task", &fields);

        if !task.milestones.is_empty() {
            let lines: Vec<String> = task
                .milestones
                .iter()
                .enumerate()
                .flat_map(|(i, m)| {
                    vec![
                        format!(
                            "{}. {} (Due: {})",
                            i + 1,
                            m.name,
                            m.due_date.map_or("-".into(), |d| d.to_string())
                        ),
                        format!("    {}", m.description),
                    ]
                })
                .collect();

            Printer::multi_line_info_with_icon(
                Icon::Info,
                "Milestones",
                lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            );
        }

        Ok(())
    }

    pub async fn remove(ctx: &impl WithContext, _id: String) -> FlowletResult<()> {
        // Look up the task first
        let task = Task::read(
            ctx.get(),
            ReadTaskInput {
                query: Query::eq("_id", _id.clone()),
                remote: false,
            },
        )
        .await?;

        let task = match task {
            Some(t) => t,
            None => {
                Printer::error(
                    Icon::Error,
                    "Task",
                    &format!("No task found with _id `{}`", _id),
                );
                return Ok(());
            }
        };

        // Confirm by task title
        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete the task `{}`?",
                task.title
            ))
            .default(false)
            .interact()?;

        if !confirm {
            Printer::info(Icon::Warning, "Aborted", "Task deletion cancelled.");
            return Ok(());
        }

        // Remove task using title
        Task::remove(
            ctx.get(),
            RemoveTaskInput {
                title: task.title.clone(),
            },
        )
        .await?;

        Printer::success(
            Icon::Trash,
            "Task",
            &format!("Deleted task `{}`.", task.title),
        );

        Ok(())
    }
}
