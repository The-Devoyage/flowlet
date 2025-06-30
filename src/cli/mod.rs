use clap::{Parser, Subcommand};

pub mod auth;
pub mod command;

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
    Run {
        #[arg(required = true)]
        name: String,
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
}

#[derive(Subcommand)]
pub enum Vars {
    /// List all variables
    Ls,

    /// Add a variable
    Add {
        #[arg(required = true)]
        key: String,
        #[arg(required = true)]
        value: String,
    },

    /// Remove a variable
    Rm {
        #[arg(required = true)]
        key: String,
    },
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
