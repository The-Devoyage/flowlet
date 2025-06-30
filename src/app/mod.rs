use crate::cli::{Auth, Commands, RootCommands, Vars};
use crate::flowlet_context::{FlowletContext, WithContext};
use crate::util::FlowletResult;

pub struct App<'a> {
    pub ctx: &'a FlowletContext,
}

impl<'a> WithContext for App<'a> {
    fn get(&self) -> &FlowletContext {
        self.ctx
    }
}

impl<'a> App<'a> {
    pub async fn run(&self, cli: crate::cli::Cli) -> FlowletResult<()> {
        match cli.root_commands {
            RootCommands::Command(commands) => match commands {
                Commands::Run { name } => crate::cli::command::Command::run(self, name).await,
                Commands::Save { name, cmd } => {
                    crate::cli::command::Command::save(self, name, cmd).await
                }
                Commands::Ls { remote } => crate::cli::command::Command::list(self, remote).await,
                Commands::Show { name } => crate::cli::command::Command::show(self, name).await,
                Commands::Rm { name } => crate::cli::command::Command::remove(self, name).await,
                Commands::Edit {name} => crate::cli::command::Command::edit(self, name).await,
            },
            RootCommands::Vars(vars) => match vars {
                Vars::Ls => {
                    println!("Listing variables...");
                    Ok(())
                }
                Vars::Add { key, value } => {
                    println!("Adding variable '{}' with value '{}'", key, value);
                    Ok(())
                }
                Vars::Rm { key } => {
                    println!("Removing variable '{}'", key);
                    Ok(())
                }
            },
            RootCommands::Auth(auth) => match auth {
                Auth::Login => crate::cli::auth::Auth::login(self).await,
                Auth::Register => crate::cli::auth::Auth::register(self).await,
                Auth::Logout => crate::cli::auth::Auth::logout(self).await
            },
        }
    }
}
