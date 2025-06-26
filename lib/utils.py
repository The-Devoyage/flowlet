import json
import re
import shlex
import os
import textwrap
import click
import tempfile
from typing import Optional
from pathlib import Path
from dotenv import load_dotenv
from pygments import highlight, lexers, formatters
from detect_secrets.core.secrets_collection import SecretsCollection
from detect_secrets.settings import default_settings


load_dotenv()  # Automatically loads from .env file


def load_env_var(key: str, default: Optional[str] = None) -> str:
    value = os.getenv(key)
    if not value:
        if default:
            return default
        raise RuntimeError(f"❌ Required environment variable '{key}' is missing.")
    return value


DATA_FILE = load_env_var("DATA_FILE", "./flowlet.json")
VARS_FILE = load_env_var("VARS_FILE", "./flowlet_vars.json")


def load_data():
    data_file = Path.home() / DATA_FILE
    if data_file.exists():
        return json.loads(data_file.read_text())
    return {}


def save_data(data):
    data_file = Path.home() / DATA_FILE
    data_file.write_text(json.dumps(data, indent=2))


def load_vars():
    vars_file = Path.home() / VARS_FILE
    if vars_file.exists():
        return json.loads(vars_file.read_text())
    return {}


def save_vars(vars_data):
    vars_file = Path.home() / VARS_FILE
    vars_file.write_text(json.dumps(vars_data, indent=2))


def remove_var(name: str):
    vars_file = Path.home() / VARS_FILE

    if not vars_file.exists():
        print("⚠️  No vars file found.")
        return

    try:
        vars_data = json.loads(vars_file.read_text())

        if name in vars_data:
            del vars_data[name]
            vars_file.write_text(json.dumps(vars_data, indent=2))
            click.secho(f"🗑️ Removed variable '{name}'", fg="yellow")
        else:
            print(f"❌ Variable '{name}' not found.")
    except json.JSONDecodeError:
        print("❌ Failed to read vars file. Is it corrupted?")


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


def boxed_wrapped_text(text, max_width=80):
    wrapped_lines = []
    for line in text.splitlines():
        wrapped_lines.extend(textwrap.wrap(line, width=max_width) or [""])
    width = max(len(line) for line in wrapped_lines)
    border = "─" * (width + 4)
    box = [f"┌{border}┐"]
    for line in wrapped_lines:
        box.append(f"│  {line.ljust(width)}  │")
    box.append(f"└{border}┘")
    return "\n".join(box)


def pretty_print_json(json_obj):
    json_str = json.dumps(json_obj, indent=2)
    colorful_json = highlight(
        json_str, lexers.JsonLexer(), formatters.TerminalFormatter()
    )
    click.echo(colorful_json)


def contains_secrets(text: str) -> bool:
    """
    Check if the text contains potential secrets using detect-secrets.

    Args:
        text: The string to scan for secrets.

    Returns:
        True if potential secrets are found, False otherwise.
    """
    with tempfile.NamedTemporaryFile(mode="w+", delete=True) as tmp:
        tmp.write(text)
        tmp.flush()

        secrets = SecretsCollection()
        with default_settings():
            secrets.scan_file(tmp.name)

        # Show secrets in a readable format
        for _, secret_list in secrets.data.items():
            for secret in secret_list:
                print(f"🔐 Potential secret found: {secret.secret_value}")

    return bool(secrets)
