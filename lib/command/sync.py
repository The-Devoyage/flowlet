import click

from lib.utils import load_data, save_data
from lib.api import find_many, find_one, update_one, insert_one


@click.command(name="pull")
def pull_commands():
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



@click.command(name="push")
@click.argument("name")
def push_command(name):
    """Push a single saved command to the remote server."""
    local_commands = load_data()

    if name not in local_commands:
        click.secho(f"❌ Command '{name}' not found in local store.", fg="red")
        return

    cmd = local_commands[name]

    # Confirm push
    if not click.confirm(f"☁️  Push command '{name}' to remote server?", default=True):
        click.echo("❌ Aborted.")
        return

    # Check if command exists remotely
    existing = find_one("command", {"name": name})

    if existing:
        # Command exists, update
        try:
            update_one("command", query={"name": name}, data={"command": cmd})
            click.secho(f"🔁 Updated remote command '{name}'.", fg="yellow")
        except Exception as e:
            click.secho(f"⚠️  Failed to update: {e}", fg="red")
    else:
        # Command doesn't exist, insert
        try:
            insert_one("command", data={"name": name, "command": cmd})
            click.secho(f"☁️  Pushed new command '{name}' to remote.", fg="green")
        except Exception as e:
            click.secho(f"⚠️  Failed to insert: {e}", fg="red")
