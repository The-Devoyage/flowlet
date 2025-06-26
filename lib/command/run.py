import json
import subprocess

import click

from lib.utils import (
    boxed_wrapped_text,
    extract_json_field,
    inject_vars,
    load_data,
    load_vars,
    pretty_print_json,
    save_vars,
)


@click.command(name="run")
@click.argument("name")
@click.option("--arg", multiple=True, help="Extra args to append to the command")
@click.option("--save-var", help="Save JSON field to var, format: var=path.to.key")
def run_command(name, arg, save_var):
    """Run a saved command."""
    data = load_data()
    cmd = data.get(name)
    if not cmd:
        click.echo(f"❌ No command saved as '{name}'")
        return

    vars_data = load_vars()
    cmd = inject_vars(cmd, vars_data)
    if arg:
        cmd += " " + " ".join(arg)

    click.echo(click.style("🚀 Running command:", fg="cyan", bold=True))
    click.echo("-" * 90)
    click.echo(click.style(boxed_wrapped_text(cmd), fg="green"))

    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)

    try:
        parsed = json.loads(result.stdout)
        if save_var:
            var_name, json_path = save_var.split("=", 1)
            value = extract_json_field(parsed, json_path)
            if value is not None:
                vars_data[var_name] = value
                save_vars(vars_data)
                click.echo(
                    click.style(
                        f"🔐 Saved '{var_name}' = '{value}'", fg="yellow", bold=True
                    )
                )
            else:
                click.echo(
                    click.style(
                        f"⚠️ Could not find value at path '{json_path}'", fg="red"
                    )
                )
        else:
            click.echo(click.style("📦 JSON Output:", fg="cyan", bold=True))
            click.echo("-" * 90)
            pretty_print_json(parsed)

    except json.JSONDecodeError:
        click.echo(click.style("📦 Output:", fg="cyan", bold=True))
        click.echo("-" * 90)
        click.echo(result.stdout)
