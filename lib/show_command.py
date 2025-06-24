import click
from lib.utils import boxed_wrapped_text, load_data

@click.command(name="show")
@click.argument("name")
def show_command(name):
    """Show the full saved command by name."""
    data = load_data()
    cmd = data.get(name)
    if not cmd:
        click.echo(f"❌ No saved command named '{name}'.")
        return

    click.echo(click.style(boxed_wrapped_text(cmd), fg="green"))

