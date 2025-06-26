import click
import textwrap
from lib.utils import load_data
from lib.api import find_many


@click.command(name="ls")
@click.option("--remote", is_flag=True, help="List commands from remote server")
def list_commands(remote):
    """List saved commands."""

    if remote:
        click.echo("☁️  Fetching remote commands...")
        data = find_many("command")

        if not data:
            click.echo("📂 No remote commands found")
            return

        # Assuming data is a list of dicts with 'name' and 'command'
        items = data
    else:
        data = load_data()
        if not data:
            click.echo("📭 No saved local commands.")
            return
        # Convert dict to list of dicts for unified processing
        items = [{"name": k, "command": v} for k, v in data.items()]

    # Determine column widths
    max_name_width = 20
    max_cmd_width = 60

    # Header
    click.echo(f"{'Name':<{max_name_width}}  Command")
    click.echo("-" * (max_name_width + 2 + max_cmd_width))

    for item in items:
        name = item.get("name", "<no name>")
        cmd = item.get("command", "<no command>")

        # Truncate long command nicely with ellipsis
        if len(cmd) > max_cmd_width:
            cmd = cmd[: max_cmd_width - 3] + "..."

        # Wrap command text to multiple lines for readability
        wrapped_cmd = textwrap.wrap(cmd, width=max_cmd_width)

        # Print first line with name, rest lines indented
        click.echo(f"{name:<{max_name_width}}  {wrapped_cmd[0]}")
        for line in wrapped_cmd[1:]:
            click.echo(" " * (max_name_width + 2) + line)
