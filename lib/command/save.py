import re
import json
import click

from lib.api import insert_one, update_one, find_one
from lib.utils import (
    contains_secrets,
    load_data,
    save_data,
    try_fix_json_like_string,
)


@click.command(name="save")
@click.argument("name")
@click.argument("cmd")
def save_command(name, cmd):
    """Save a command locally and remotely."""

    # 🚨 Warn about secrets
    if contains_secrets(cmd):
        click.secho(
            "⚠️  Potential secret detected in this command!", fg="yellow", bold=True
        )
        click.secho(
            "🔐 It looks like this command might include a token, key, or password.\n",
            fg="bright_yellow",
        )
        if not click.confirm("❓ Do you still want to save this command?", default=False):
            click.echo("❌ Aborted.")
            return

    # 🧹 Try to fix malformed JSON in -d argument
    json_data_match = re.search(r"-d\\s+'([^']+)'", cmd)
    if json_data_match:
        raw_json_str = json_data_match.group(1)
        try:
            json.loads(raw_json_str)
        except json.JSONDecodeError:
            fixed_json = try_fix_json_like_string(raw_json_str)
            if fixed_json:
                click.echo(f"⚠️  Fixed invalid JSON:\n  From: {raw_json_str}\n  To:   {fixed_json}")
                cmd = cmd.replace(raw_json_str, fixed_json)

    # 💾 Save locally
    data = load_data()
    data[name] = cmd
    save_data(data)
    click.echo(f"✅ Locally saved command '{name}'.")

    # ☁️ Save remotely (insert or update)
    existing = find_one("command", {"name": name})
    try:
        if existing:
            update_one("command", query={"name": name}, data={"command": cmd})
            click.secho(f"🔁 Updated remote command '{name}'.", fg="yellow")
        else:
            insert_one("command", data={"name": name, "command": cmd})
            click.secho(f"☁️  Inserted new remote command '{name}'.", fg="green")
    except Exception as e:
        click.secho(f"⚠️  Remote save failed: {e}", fg="red")
