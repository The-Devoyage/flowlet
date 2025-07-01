use deeb::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    flowlet_context::FlowletContext,
    printer::{Icon, Printer},
    util::FlowletResult,
};

use super::Api;

#[derive(Collection, Deserialize, Serialize)]
pub struct Variable {
    pub _id: ulid::Ulid,
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct CreateVariableInput {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct UpdateVariableInput {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct RemoveVariableInput {
    pub name: String,
}

#[derive(Serialize)]
pub struct ReadVariableInput {
    pub query: Query,
}

#[derive(Serialize)]
pub struct ListVariableInput {
    pub query: Query,
}

#[derive(Debug, Error)]
pub enum VariableApiError {
    #[error("Failed to create variable.")]
    CreateFailed,

    #[error("Failed to read variable.")]
    ReadFailed,

    #[error("Failed to update variable.")]
    UpdateFailed,

    #[error("Failed to delete variable.")]
    DeleteFailed,

    #[error("No variables found.")]
    NoVariablesFound,

    #[error("Variable not found.")]
    VariableNotFound,
}

impl Api for Variable {
    type CreateInput = CreateVariableInput;
    async fn create(
        flowlet_context: &FlowletContext,
        input: Self::CreateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let variable = Variable {
            _id: ulid::Ulid::new(),
            name: input.name,
            value: input.value,
        };

        let saved = Variable::insert_one(deeb, variable, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                VariableApiError::CreateFailed
            })?;

        Printer::success(Icon::Local, "Variable", "Saved to local store.");

        Ok(saved)
    }

    type ReadInput = ReadVariableInput;
    async fn read(
        flowlet_context: &FlowletContext,
        input: Self::ReadInput,
    ) -> FlowletResult<Option<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let result = Variable::find_one(deeb, input.query.clone(), None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                VariableApiError::ReadFailed
            })?;

        Ok(result)
    }

    type UpdateInput = UpdateVariableInput;
    async fn update(
        flowlet_context: &FlowletContext,
        input: Self::UpdateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let query = Query::eq("name", input.name.clone());

        let updated = Variable::update_one::<UpdateVariableInput>(deeb, query, input, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                VariableApiError::UpdateFailed
            })?;

        if let Some(var) = updated {
            Printer::success(Icon::Local, "Variable", "Updated successfully.");
            Ok(var)
        } else {
            Err(Box::new(VariableApiError::VariableNotFound))
        }
    }

    type ListInput = ListVariableInput;
    async fn list(
        flowlet_context: &FlowletContext,
        input: Self::ListInput,
    ) -> FlowletResult<Vec<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let result = Variable::find_many(deeb, input.query, None, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                VariableApiError::ReadFailed
            })?;

        if let Some(vars) = result {
            Ok(vars)
        } else {
            Err(Box::new(VariableApiError::NoVariablesFound))
        }
    }

    type RemoveInput = RemoveVariableInput;
    async fn remove(
        flowlet_context: &FlowletContext,
        input: Self::RemoveInput,
    ) -> FlowletResult<bool> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let query = Query::eq("name", input.name);

        let deleted = Variable::delete_one(deeb, query, None).await.map_err(|e| {
            log::error!("{:?}", e);
            VariableApiError::DeleteFailed
        })?;

        if let Some(result) = deleted {
            if result {
                Printer::success(Icon::Trash, "Variable", "Deleted successfully.");
                Ok(true)
            } else {
                Err(Box::new(VariableApiError::VariableNotFound))
            }
        } else {
            Err(Box::new(VariableApiError::DeleteFailed))
        }
    }
}
