use clap::{Parser, Subcommand};

pub mod auth;

#[derive(Parser)]
#[command(name = "flowlet")]
#[command(about = "🌊 The CLI to manage developer flow")]
pub struct Cli {
    #[command(subcommand)]
    pub root_commands: RootCommands,
}

#[derive(Subcommand)]
pub enum RootCommands {
    /// Manage saved commands
    #[command(subcommand)]
    Command(Commands),

    /// Manage saved variables
    #[command(subcommand)]
    Vars(Vars),

    /// Register, login, and  logout.
    #[command(subcommand)]
    Auth(Auth),
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a saved command
    Run { name: String },
    /// Save a command
    Save {
        name: String,
        #[arg(required = true)]
        cmd: Vec<String>, // handles multi-word shell command
    },
}

#[derive(Subcommand)]
pub enum Vars {
    /// List all variables
    Ls,

    /// Add a variable
    Add { key: String, value: String },

    /// Remove a variable
    Rm { key: String },
}

#[derive(Subcommand)]
pub enum Auth {
    /// Register as a Flowlet user.
    Register,

    /// Login/Authenticate
    Login,

    /// End session
    Logout,
}
