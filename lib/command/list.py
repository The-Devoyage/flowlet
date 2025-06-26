import click
import textwrap

import requests

from lib.utils import load_data
from lib.api import TokenMissingError, find_many


@click.command(name="ls")
@click.option("--remote", is_flag=True, help="List commands from remote server")
def list_commands(remote):
    """List saved commands."""

    if remote:
        click.echo("☁️  Fetching remote commands...")
        try:
            data = find_many("command")
        except TokenMissingError as e:  # token missing
            click.secho(f"⚠️  {e}", fg="yellow")
            return
        except requests.HTTPError as e:
            click.secho(f"🚨 HTTP error: {e}", fg="red")
            click.secho(f"❌ Server response: {e.response.text if e.response else e}", fg="red")
            return
        except requests.RequestException as e:
            click.secho(f"❌ Failed to fetch remote commands: {e}", fg="red")
            return

        if not data:
            click.echo("📂 No remote commands found")
            return

        items = data
    else:
        data = load_data()
        if not data:
            click.echo("📭 No saved local commands.")
            return

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

        if len(cmd) > max_cmd_width:
            cmd = cmd[: max_cmd_width - 3] + "..."

        wrapped_cmd = textwrap.wrap(cmd, width=max_cmd_width)
        click.echo(f"{name:<{max_name_width}}  {wrapped_cmd[0]}")
        for line in wrapped_cmd[1:]:
            click.echo(" " * (max_name_width + 2) + line)
