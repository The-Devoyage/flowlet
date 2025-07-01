use deeb::Query;
use dialoguer::Confirm;
use thiserror::Error;

use crate::{
    flowlet_context::WithContext,
    flowlet_db::models::{
        self, Api,
        variable::{CreateVariableInput, ListVariableInput, RemoveVariableInput},
    },
    printer::{Icon, Printer},
    util::FlowletResult,
};

#[derive(Debug, Error)]
pub enum CliVariableError {
    #[error("Variable not found.")]
    VariableNotFound,

    #[error("Variable creation failed.")]
    CreateFailed,
}

pub struct Variable;

impl Variable {
    pub async fn list(ctx: &impl WithContext) -> FlowletResult<()> {
        let variables =
            models::variable::Variable::list(ctx.get(), ListVariableInput { query: Query::All })
                .await?;

        if variables.is_empty() {
            Printer::warning(Icon::Warning, "Empty", "No variables found.");
            return Ok(());
        }

        let rows: Vec<Vec<String>> = variables
            .into_iter()
            .map(|var| vec![var.name, var.value])
            .collect();

        Printer::success(Icon::Success, "Variables", "List of stored variables:");
        Printer::table(vec!["Name", "Value"], rows);
        Ok(())
    }

    pub async fn add(ctx: &impl WithContext, name: String, value: String) -> FlowletResult<()> {
        models::variable::Variable::create(
            ctx.get(),
            CreateVariableInput {
                name: name.clone(),
                value,
            },
        )
        .await?;

        Printer::success(
            Icon::Success,
            "Saved",
            &format!("Saved variable `{}`", name),
        );
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

        models::variable::Variable::remove(ctx.get(), RemoveVariableInput { name: name.clone() })
            .await?;

        Printer::success(
            Icon::Trash,
            "Removed",
            &format!("Deleted variable `{}`", name),
        );
        Ok(())
    }
}
