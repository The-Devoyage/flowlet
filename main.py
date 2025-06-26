from pathlib import Path

import click

from lib.command import commands_group
from lib.command.run import run_command
from lib.vars import vars_group
from lib.auth import auth_group

DATA_FILE = Path.home() / ".flowlet.json"
VARS_FILE = Path.home() / ".flowlet_vars.json"


class DefaultGroup(click.Group):
    def get_command(self, ctx, cmd_name):
        cmd = click.Group.get_command(self, ctx, cmd_name)
        if cmd:
            return cmd

        # If command not found, treat it as `flowlet command run <cmd_name>`
        # Create a fake context for run_command with cmd_name as argument
        def default_run():
            # Run the existing run command with cmd_name as argument
            ctx.invoke(run_command, name=cmd_name)

        return click.Command(cmd_name, callback=default_run)


@click.group(cls=DefaultGroup)
def cli():
    pass


cli.add_command(commands_group)
cli.add_command(vars_group)
cli.add_command(auth_group)

if __name__ == "__main__":
    cli()
