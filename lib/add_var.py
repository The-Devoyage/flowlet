import click
from lib.utils import load_vars, save_vars

@click.command(name="add-var")
@click.argument("key")
@click.argument("value")
def add_var(key, value):
    """Manually save a variable to use with ${key} placeholders."""
    vars_data = load_vars()
    vars_data[key] = value
    save_vars(vars_data)
    click.echo(f"🔐 Manually saved variable '{key}' = '{value}'")
