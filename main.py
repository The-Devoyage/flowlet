import json
import re
import shlex
import subprocess
from pathlib import Path

import click

DATA_FILE = Path.home() / ".flowlet.json"
VARS_FILE = Path.home() / ".flowlet_vars.json"


# --- File Helpers ---
def load_data():
    if DATA_FILE.exists():
        return json.loads(DATA_FILE.read_text())
    return {}


def save_data(data):
    DATA_FILE.write_text(json.dumps(data, indent=2))


def load_vars():
    if VARS_FILE.exists():
        return json.loads(VARS_FILE.read_text())
    return {}


def save_vars(vars_data):
    VARS_FILE.write_text(json.dumps(vars_data, indent=2))


def inject_vars(cmd, vars_data):
    def replacer(match):
        var_name = match.group(1)
        value = vars_data.get(var_name)
        if value is None:
            click.secho(
                f"⚠️  Warning: Variable '{var_name}' not found in vars file.",
                fg="yellow",
            )
            return match.group(0)
        return str(value)

    return re.sub(r"\$\{(\w+)}", replacer, cmd)


# --- JSON Path Helper ---
def extract_json_field(data, path):
    keys = path.split(".")
    for key in keys:
        if isinstance(data, dict):
            data = data.get(key)
        else:
            return None
    return data


def try_fix_json_like_string(s):
    try:
        json.loads(s)
        return s
    except Exception:
        pass

    try:
        fixed = re.sub(r"([{,]\s*)(\w+)\s*:", r'\1"\2":', s)
        fixed = re.sub(r":\s*([\w@.\-]+)", r': "\1"', fixed)
        json.loads(fixed)
        return fixed
    except Exception:
        return None


def wrap_shell_command(cmd, width=100, indent="  "):
    try:
        parts = shlex.split(cmd)
    except ValueError:
        return cmd

    lines = []
    current = indent

    for part in parts:
        part = shlex.quote(part)
        if len(current) + len(part) + 1 > width:
            lines.append(current + " \\")
            current = indent + part
        else:
            current += " " + part if current.strip() else indent + part

    lines.append(current)
    return "\n".join(lines)


@click.group()
def cli():
    pass


@cli.command()
@click.argument("name")
@click.argument("cmd")
def save(name, cmd):
    """Save a command under NAME."""
    json_data_match = re.search(r"-d\s+'([^']+)'", cmd)
    if json_data_match:
        raw_json_str = json_data_match.group(1)
        try:
            json.loads(raw_json_str)
        except json.JSONDecodeError:
            fixed_json = try_fix_json_like_string(raw_json_str)
            if fixed_json:
                click.echo("⚠️  Fixed invalid JSON:")
                click.echo(f"  From: {raw_json_str}")
                click.echo(f"  To:   {fixed_json}")
                cmd = cmd.replace(raw_json_str, fixed_json)
            else:
                click.echo("❌ Invalid JSON could not be auto-corrected.")

    if len(cmd) > 100:
        click.echo("📦 Wrapping long command for readability.")
        cmd = wrap_shell_command(cmd)

    data = load_data()
    data[name] = cmd
    save_data(data)
    click.echo(f"✅ Saved command '{name}'.")


@cli.command()
@click.argument("name")
@click.option("--arg", multiple=True, help="Extra args to append to the command")
@click.option("--save-var", help="Save JSON field to var, format: var=path.to.key")
def run(name, arg, save_var):
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

    click.echo(f"🚀 Running: {cmd}")
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)

    try:
        parsed = json.loads(result.stdout)
        if save_var:
            var_name, json_path = save_var.split("=", 1)
            value = extract_json_field(parsed, json_path)
            if value is not None:
                vars_data[var_name] = value
                save_vars(vars_data)
                click.echo(f"🔐 Saved '{var_name}' = '{value}'")
            else:
                click.echo(f"⚠️  Could not find value at path '{json_path}'")
        else:
            click.echo(json.dumps(parsed, indent=2))
    except json.JSONDecodeError:
        click.echo(result.stdout)


@cli.command(name="list")
def list():
    """List saved commands."""
    data = load_data()
    if not data:
        click.echo("📭 No saved commands.")
        return
    for name, cmd in data.items():
        click.echo(f"- {name}: {cmd}")


@cli.command()
def vars():
    """List saved variables."""
    vars_data = load_vars()
    if not vars_data:
        click.echo("📭 No saved vars.")
    else:
        click.echo(json.dumps(vars_data, indent=2))

cli.add_command(list, name="ls")
cli.add_command(run, name="r")
cli.add_command(save, name="s")

if __name__ == "__main__":
    cli()
