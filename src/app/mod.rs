use crate::cli::{Auth, Commands, RootCommands, Vars};
use crate::cli::{command::Command, variable::Variable};
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
                Commands::Run {
                    name,
                    save_var,
                    json_path,
                } => Command::run(self, name, save_var, json_path).await,
                Commands::Save { name, cmd } => Command::save(self, name, cmd).await,
                Commands::Ls { remote } => Command::list(self, remote).await,
                Commands::Show { name } => Command::show(self, name).await,
                Commands::Rm { name } => Command::remove(self, name).await,
                Commands::Edit { name } => Command::edit(self, name).await,
                Commands::Push { name } => Command::push(self, name).await,
                Commands::Pull { name } => Command::pull(self, name).await,
            },
            RootCommands::Vars(vars) => match vars {
                Vars::Ls => Variable::list(self).await,
                Vars::Add { key, value } => Variable::add(self, key, value).await,
                Vars::Rm { key } => Variable::remove(self, key).await,
            },
            RootCommands::Auth(auth) => match auth {
                Auth::Login => crate::cli::auth::Auth::login(self).await,
                Auth::Register => crate::cli::auth::Auth::register(self).await,
                Auth::Logout => crate::cli::auth::Auth::logout(self).await,
            },
        }
    }
}
