use crate::{
    flowlet_context::WithContext,
    flowlet_db::models::{
        self, Api,
        command::{CreateCommandInput, ReadCommandInput, UpdateCommandInput},
    },
    printer::Printer,
    util::FlowletResult,
};

pub struct Command;

impl Command {
    pub async fn save(ctx: &impl WithContext, name: String, cmd: Vec<String>) -> FlowletResult<()> {
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
}
