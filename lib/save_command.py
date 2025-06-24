import re
import json
import click

from lib.api import insert
from lib.utils import load_data, save_data, try_fix_json_like_string 


@click.command(name="save")
@click.argument("name")
@click.argument("cmd")
def save_command(name, cmd):
    """Save a command locally and remotely."""

    # Optional: fix malformed JSON in -d argument
    json_data_match = re.search(r"-d\\s+'([^']+)'", cmd)
    if json_data_match:
        raw_json_str = json_data_match.group(1)
        try:
            json.loads(raw_json_str)
        except json.JSONDecodeError:
            fixed_json = try_fix_json_like_string(raw_json_str)
            if fixed_json:
                click.echo(
                    f"⚠️  Fixed invalid JSON:\n  From: {raw_json_str}\n  To:   {fixed_json}"
                )
                cmd = cmd.replace(raw_json_str, fixed_json)

    # Save locally
    data = load_data()
    data[name] = cmd
    save_data(data)
    click.echo(f"✅ Locally saved command '{name}'.")

    # Save remotely
    insert("command", data={"name": name, "command": cmd})
