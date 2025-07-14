use clap::{Parser, Subcommand};

pub mod auth;
pub mod command;
pub mod project;
pub mod task;
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

    /// Manage projects
    #[command(subcommand)]
    Project(Project),

    /// Manage Tasks
    #[command(subcommand)]
    Task(Task),

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
    // List commands
    Ls {
        #[arg(long)]
        remote: bool,
        #[arg(long)]
        global: bool,
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

#[derive(Subcommand)]
pub enum Project {
    /// Create a new project.
    New,

    /// Remove a project by name
    Rm { name: String },

    /// List all projects
    Ls,
}

#[derive(Subcommand)]
pub enum Task {
    /// Create a new task.
    New,

    /// Remove a task by _id. 
    Rm { _id: String },

    /// List all projects
    Ls {
        #[arg(long)]
        remote: bool,
        #[arg(long)]
        global: bool,
    },

    /// View the details of a task
    Show { _id: String },
    
    /// Edit the details of a task
    Edit { _id: String },
}
