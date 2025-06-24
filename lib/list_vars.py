import json
import click

from lib.utils import load_vars

@click.command(name="vars")
def list_vars():
    """List saved variables."""
    vars_data = load_vars()
    if not vars_data:
        click.echo("📭 No saved vars.")
    else:
        click.echo(json.dumps(vars_data, indent=2))

