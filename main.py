from pathlib import Path

import click

from lib.command import commands_group
from lib.vars import vars_group 
from lib.auth_command import login_user, logout_user, register_user

DATA_FILE = Path.home() / ".flowlet.json"
VARS_FILE = Path.home() / ".flowlet_vars.json"


@click.group()
def cli():
    pass


cli.add_command(commands_group)
cli.add_command(vars_group)
cli.add_command(login_user)
cli.add_command(register_user)
cli.add_command(logout_user)

if __name__ == "__main__":
    cli()
