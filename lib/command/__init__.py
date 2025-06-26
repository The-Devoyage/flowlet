import click
from click_aliases import ClickAliasedGroup

from lib.command.list import list_commands
from lib.command.remove import remove_command
from lib.command.run import run_command
from lib.command.save import save_command
from lib.command.show import show_command
from lib.command.sync import pull_commands, push_command

@click.group(name="command", cls=ClickAliasedGroup)
def commands_group():
    """Manage commands locally and in the cloud."""
    pass

commands_group.add_command(list_commands)
commands_group.add_command(run_command)
commands_group.add_command(save_command)
commands_group.add_command(show_command)
commands_group.add_command(remove_command)
commands_group.add_command(pull_commands)
commands_group.add_command(push_command)
