import click
from lib.utils import load_data, save_data
from lib.api import delete_one


@click.command(name="rm")
@click.argument("name")
def remove_command(name):
    """Remove a saved command locally and remotely."""
    data = load_data()

    if name not in data:
        click.secho(f"❌ Command '{name}' not found.", fg="red")
        return

    # Confirm removal
    if not click.confirm(f"❓ Are you sure you want to delete '{name}'?", default=False):
        click.echo("❌ Aborted.")
        return

    # Remove locally
    del data[name]
    save_data(data)
    click.secho(f"🗑️  Locally deleted command '{name}'", fg="yellow")

    # Remove remotely
    try:
        delete_one("command", {"Eq": ["name", name]})
        click.secho("☁️  Remote command removed.", fg="cyan")
    except Exception as e:
        click.secho(f"⚠️  Could not delete from server: {e}", fg="red")
