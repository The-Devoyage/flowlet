use app::App;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use flowlet_context::FlowletContext;
use util::FlowletResult;

pub mod app;
pub mod cli;
pub mod flowlet_context;
pub mod flowlet_db;
pub mod util;
pub mod api_client;
pub mod printer;

#[tokio::main]
async fn main() -> FlowletResult<()> {
    pretty_env_logger::init();
    let ctx = FlowletContext::new().await?;
    let app = App { ctx: &ctx };
    let cli = Cli::parse();

    if let Err(e) = app.run(cli).await {
        eprintln!("{}", e.to_string().red());
    }

    Ok(())
}
