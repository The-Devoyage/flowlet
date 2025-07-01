use clap::{Parser, Subcommand};

pub mod auth;
pub mod command;
pub mod variable;

#[derive(Parser)]
#[command(name = "flowlet")]
#[command(about = "🌊 The CLI for developer flow.")]
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

    /// Catch-all for unknown commands
    #[command(external_subcommand)]
    Unknown(Vec<String>),
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a saved command
    Run {
        #[arg(required = true)]
        name: String,

        #[arg(long)]
        save_var: Option<String>,

        #[arg(long)]
        json_path: Option<String>,
    },
    /// Save a command
    Save {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        cmd: String, // handles multi-word shell command
    },
    // List local commands
    Ls {
        #[arg(long)]
        remote: bool,
    },
    Show {
        name: String,
    },
    Rm {
        name: String,
    },
    Edit {
        name: String,
    },
    Push {
        name: String,
    },
    Pull {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum Vars {
    /// List all variables
    Ls,

    /// Add a variable
    Set { key: String, value: String },

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
