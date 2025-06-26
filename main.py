from pathlib import Path

import click

from lib.vars import vars_group 
from lib.auth_command import login_user, logout_user, register_user
from lib.list_commands import list_commands
from lib.run_command import run_command
from lib.save_command import save_command
from lib.show_command import show_command
from lib.sync import sync_command

DATA_FILE = Path.home() / ".flowlet.json"
VARS_FILE = Path.home() / ".flowlet_vars.json"


@click.group()
def cli():
    pass


cli.add_command(list_commands)
cli.add_command(run_command)
cli.add_command(save_command)
cli.add_command(vars_group)
cli.add_command(sync_command)
cli.add_command(show_command)
cli.add_command(login_user)
cli.add_command(register_user)
cli.add_command(logout_user)

if __name__ == "__main__":
    cli()
