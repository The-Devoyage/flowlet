import click

from lib.utils import load_data, save_data
from lib.api import find_many


@click.command(name="sync")
def sync_command():
    """Sync remote commands to your local store."""
    click.echo("☁️  Syncing remote commands to local...")

    remote_commands = find_many("command")
    local_commands = load_data()

    changes = 0
    for item in remote_commands:
        name = item.get("name")
        cmd = item.get("command")

        if name not in local_commands:
            click.echo(f"➕ Adding new command: {name}")
            local_commands[name] = cmd
            changes += 1
        elif local_commands[name] != cmd:
            click.echo(f"🔁 Updating changed command: {name}")
            local_commands[name] = cmd
            changes += 1

    if changes > 0:
        save_data(local_commands)
        click.echo(f"✅ Synced {changes} command(s) from server.")
    else:
        click.echo("✅ Already up to date.")
