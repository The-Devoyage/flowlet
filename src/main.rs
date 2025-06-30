use app::App;
use clap::Parser;
use cli::Cli;
use flowlet_context::FlowletContext;
use printer::{Icon, Printer};
use util::FlowletResult;

pub mod api_client;
pub mod app;
pub mod cli;
pub mod flowlet_context;
pub mod flowlet_db;
pub mod printer;
pub mod util;

#[tokio::main]
async fn main() -> FlowletResult<()> {
    pretty_env_logger::init();
    let ctx = FlowletContext::new().await?;
    let app = App { ctx: &ctx };
    let cli = Cli::parse();

    if let Err(e) = app.run(cli).await {
        Printer::error(Icon::Error, "Error", &e.to_string());
    }

    Ok(())
}
