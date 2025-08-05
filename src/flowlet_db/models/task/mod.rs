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

#[derive(Collection, Deserialize, Serialize, Clone)]
pub struct Task {
    pub _id: ulid::Ulid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub project: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub tags: Vec<String>,
    pub milestones: Vec<Milestone>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Milestone {
    pub name: String,
    pub description: String,
    pub due_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Done => "Done",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub project: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub tags: Vec<String>,
    pub milestones: Vec<Milestone>,
}

#[derive(Serialize)]
pub struct UpdateTaskInput {
    pub title: String,
    pub status: Option<TaskStatus>,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub tags: Option<Vec<String>>,
    pub milestones: Option<Vec<Milestone>>,
}

#[derive(Serialize)]
pub struct RemoveTaskInput {
    pub title: String,
}

#[derive(Serialize)]
pub struct ReadTaskInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Serialize)]
pub struct ListTaskInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Debug, Error)]
pub enum TaskApiError {
    #[error("Failed to create task.")]
    CreateFailed,

    #[error("Failed to read task.")]
    ReadFailed,

    #[error("Failed to update task.")]
    UpdateFailed,

    #[error("Failed to delete task.")]
    DeleteFailed,

    #[error("No tasks found.")]
    NoTasksFound,

    #[error("Task not found.")]
    TaskNotFound,
}

impl Api for Task {
    type CreateInput = CreateTaskInput;
    async fn create(ctx: &FlowletContext, input: Self::CreateInput) -> FlowletResult<Self> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let task = Task {
            _id: ulid::Ulid::new(),
            title: input.title,
            description: input.description,
            status: TaskStatus::Todo,
            project: input.project,
            due_date: input.due_date,
            tags: input.tags,
            milestones: input.milestones,
        };

        let saved = Task::insert_one(deeb, task, None).await.map_err(|e| {
            log::error!("{:?}", e);
            TaskApiError::CreateFailed
        })?;

        Printer::success(Icon::Local, "Task", "Saved to local store.");

        // Try syncing to remote
        let remote_saved = client.post::<_, Task>("/insert-one/task", &saved).await;

        if let Err(e) = &remote_saved {
            Printer::warning(
                Icon::Cloud,
                "Remote",
                &format!("Failed to sync task to remote: {:?}", e),
            );
        } else {
            Printer::success(Icon::Cloud, "Task", "Saved to cloud.");
        }

        Ok(saved)
    }

    type ReadInput = ReadTaskInput;
    async fn read(ctx: &FlowletContext, input: Self::ReadInput) -> FlowletResult<Option<Self>> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        if input.remote {
            let res = client
                .post::<_, Task>("/find_one/task", &json!({"query": input.query}))
                .await?;

            return Ok(res.data);
        }

        let task = Task::find_one(deeb, input.query.clone(), None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                TaskApiError::ReadFailed
            })?;

        Ok(task)
    }

    type UpdateInput = UpdateTaskInput;
    async fn update(ctx: &FlowletContext, input: Self::UpdateInput) -> FlowletResult<Self> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let query = Query::eq("title", input.title.clone());

        let updated = Task::update_one::<UpdateTaskInput>(deeb, query.clone(), input, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                TaskApiError::UpdateFailed
            })?;

        if updated.is_none() {
            return Err(Box::new(TaskApiError::TaskNotFound));
        }

        let task = updated.unwrap();

        let _ = client
            .post::<_, Task>(
                "/update-one/task",
                &json!({
                    "query": query,
                    "document": task.clone()
                }),
            )
            .await
            .map_err(|_| {
                Printer::warning(Icon::Cloud, "Remote", "Failed to update project remotely.");
            });

        Printer::success(Icon::Local, "Task", "Updated successfully.");
        Ok(task)
    }

    type ListInput = ListTaskInput;
    async fn list(ctx: &FlowletContext, input: Self::ListInput) -> FlowletResult<Vec<Self>> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        if input.remote {
            let res = client
                .post::<_, Vec<Task>>("/find-many/task", &json!({ "query": input.query }))
                .await?;

            return Ok(res.data.unwrap_or_default());
        }

        let result = Task::find_many(deeb, input.query, None, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                TaskApiError::ReadFailed
            })?;

        if let Some(tasks) = result {
            Ok(tasks)
        } else {
            Err(Box::new(TaskApiError::NoTasksFound))
        }
    }

    type RemoveInput = RemoveTaskInput;
    async fn remove(ctx: &FlowletContext, input: Self::RemoveInput) -> FlowletResult<bool> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let query = Query::eq("title", input.title);

        let _ = client
            .post::<_, bool>("/delete-one/task", &json!({ "query": query.clone() }))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                e
            });

        let deleted = Task::delete_one(deeb, query, None).await.map_err(|e| {
            log::error!("{:?}", e);
            TaskApiError::DeleteFailed
        })?;

        if let Some(true) = deleted {
            Printer::success(Icon::Trash, "Task", "Deleted successfully.");
            Ok(true)
        } else {
            Err(Box::new(TaskApiError::TaskNotFound))
        }
    }
}
