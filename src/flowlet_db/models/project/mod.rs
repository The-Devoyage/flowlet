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
pub struct Project {
    pub _id: ulid::Ulid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct UpdateProjectInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ReadProjectInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Serialize)]
pub struct ListProjectInput {
    pub query: Query,
    pub remote: bool,
}

#[derive(Serialize)]
pub struct RemoveProjectInput {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ProjectApiError {
    #[error("Failed to save project.")]
    SaveFailed,

    #[error("Failed to update project.")]
    UpdateFailed,

    #[error("Project not found.")]
    ProjectNotFound,

    #[error("Failed to delete project.")]
    DeleteFailed,
}

impl Api for Project {
    type CreateInput = CreateProjectInput;

    async fn create(ctx: &FlowletContext, input: Self::CreateInput) -> FlowletResult<Self> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let project = Project {
            _id: ulid::Ulid::new(),
            name: input.name,
            description: input.description,
        };

        let saved = Project::insert_one(deeb, project.clone(), None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                ProjectApiError::SaveFailed
            })?;

        // Try syncing to remote
        let _ = client
            .post::<_, Project>("/insert-one/project", &saved)
            .await
            .map_err(|_| {
                Printer::warning(Icon::Cloud, "Remote", "Failed to sync project to remote.")
            });

        Printer::success(Icon::Local, "Project", "Created successfully.");
        Ok(saved)
    }

    type ReadInput = ReadProjectInput;

    async fn read(ctx: &FlowletContext, input: Self::ReadInput) -> FlowletResult<Option<Self>> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        if input.remote {
            let res = client
                .post::<_, Project>("/find-one/project", &json!({ "query": input.query }))
                .await?;

            return Ok(res.data);
        }

        Ok(Project::find_one(deeb, input.query, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                ProjectApiError::ProjectNotFound
            })?)
    }

    type UpdateInput = UpdateProjectInput;

    async fn update(ctx: &FlowletContext, input: Self::UpdateInput) -> FlowletResult<Self> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let query = Query::eq("name", input.name.clone());

        let updated =
            Project::update_one::<UpdateProjectInput>(deeb, query.clone(), input.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    ProjectApiError::UpdateFailed
                })?;

        if updated.is_none() {
            return Err(Box::new(ProjectApiError::ProjectNotFound));
        }

        let project = updated.unwrap();

        let _ = client
            .post::<_, Project>(
                "/update-one/project",
                &json!({
                    "query": query,
                    "document": project.clone()
                }),
            )
            .await
            .map_err(|_| {
                Printer::warning(Icon::Cloud, "Remote", "Failed to update project remotely.");
            });

        Printer::success(Icon::Local, "Project", "Updated successfully.");
        Ok(project)
    }

    type ListInput = ListProjectInput;

    async fn list(ctx: &FlowletContext, input: Self::ListInput) -> FlowletResult<Vec<Self>> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        if input.remote {
            let res = client
                .post::<_, Vec<Project>>("/find-many/project", &json!({ "query": input.query }))
                .await?;

            return Ok(res.data.unwrap_or_default());
        }

        let res = Project::find_many(deeb, input.query, None, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                ProjectApiError::ProjectNotFound
            })?;

        Ok(res.unwrap_or_default())
    }

    type RemoveInput = RemoveProjectInput;

    async fn remove(ctx: &FlowletContext, input: Self::RemoveInput) -> FlowletResult<bool> {
        let deeb = &ctx.flowlet_db.deeb;
        let client = &ctx.api_client;

        let query = Query::eq("name", input.name.clone());

        let _ = client
            .post::<_, bool>("/delete-one/project", &json!({ "query": query.clone() }))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                e
            });

        let deleted = Project::delete_one(deeb, query, None).await.map_err(|e| {
            log::error!("{:?}", e);
            ProjectApiError::DeleteFailed
        })?;

        if deleted.is_none() {
            return Err(Box::new(ProjectApiError::ProjectNotFound));
        }

        Printer::success(Icon::Trash, "Project", "Deleted.");
        Ok(true)
    }
}
