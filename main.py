from pathlib import Path

import click

from lib.add_var import add_var
from lib.list_vars import list_vars
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
cli.add_command(list_vars)
cli.add_command(add_var)
cli.add_command(sync_command)
cli.add_command(show_command)

if __name__ == "__main__":
    cli()
