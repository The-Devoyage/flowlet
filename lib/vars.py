# commands/vars_group.py
import json
import click
from click_aliases import ClickAliasedGroup

from lib.utils import load_vars, save_vars, remove_var


@click.group(name="vars", cls=ClickAliasedGroup)
def vars_group():
    """Manage saved variables."""
    pass


@vars_group.command(aliases=["list", "ls"])
def list_vars():
    """List all saved variables."""
    vars_data = load_vars()
    if not vars_data:
        click.echo("📭 No saved vars.")
    else:
        click.echo(json.dumps(vars_data, indent=2))


@vars_group.command(name="add")
@click.argument("key")
@click.argument("value")
def add_var(key, value):
    """Add a new variable."""
    vars_data = load_vars()
    vars_data[key] = value
    save_vars(vars_data)
    click.secho(f"🔐 Saved variable '{key}' = '{value}'", fg="green")


@vars_group.command(name="rm")
@click.argument("key")
def remove_var_cmd(key):
    """Remove a saved variable."""
    vars_data = load_vars()
    if key in vars_data:
        remove_var(key)
        click.secho(f"🗑️ Removed variable '{key}'", fg="yellow")
    else:
        click.secho(f"⚠️ Variable '{key}' not found.", fg="red")
