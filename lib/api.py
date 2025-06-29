import requests
import click

from lib.utils import extract_json_field, load_env_var, load_vars, save_vars

SERVER_URL = load_env_var("SERVER_URL", "http://localhost:8080")


class TokenMissingError(Exception):
    """Exception raised when the flowlet token is missing."""

    pass


def get_flowlet_token():
    vars_data = load_vars()
    token = vars_data.get("flowlet_token")
    if not token:
        raise TokenMissingError("No 'flowlet_token' found. Please login first.")
    return token


def insert_one(entity: str, data: dict):
    """Send a command to the remote server."""
    try:
        response = requests.post(
            f"{SERVER_URL}/insert-one/{entity}",
            json=data,
            timeout=5,
        )
        if response.ok:
            click.echo("☁️  Inserted document.")
        else:
            click.echo(f"⚠️  Server error: {response.status_code} - {response.text}")
    except requests.RequestException as e:
        click.echo(f"❌ Failed to sync with remote server: {e}")


def find_one(entity: str, query: dict):
    try:
        response = requests.post(
            f"http://localhost:8080/find-one/{entity}", json={"query": query}, timeout=5
        )
        if response.ok:
            return response.json()
        return None
    except Exception:
        return None


def update_one(entity: str, query: dict, data: dict):
    return requests.post(
        f"{SERVER_URL}/update-one/{entity}",
        json={"query": query, "document": data},
        timeout=5,
    )


def find_many(entity: str) -> list[dict]:
    token = get_flowlet_token()  # may raise ValueError if missing
    response = requests.post(
        f"{SERVER_URL}/find-many/{entity}",
        json={},
        timeout=5,
        headers={"Authorization": f"Bearer {token}"},
    )
    response.raise_for_status()
    return response.json().get("data", [])


def register(email: str, password: str):
    """Register on the remote server."""
    try:
        response = requests.post(
            "{SERVER_URL}/auth/register",
            json={"email": email, "password": password},
            timeout=5,
        )
        if response.ok:
            click.echo("☁️  User registered.")
        else:
            click.echo(f"⚠️  Server error: {response.status_code} - {response.text}")
    except requests.RequestException as e:
        click.echo(f"❌ Failed to register user: {e}")


def login(email: str, password: str):
    """Login to the remote server and save the token."""
    try:
        response = requests.post(
            f"{SERVER_URL}/auth/login",
            json={"email": email, "password": password},
            timeout=5,
        )

        if response.ok:
            click.echo("☁️  User authorized.")

            try:
                parsed = response.json()
                token = extract_json_field(parsed, "data.token")
                if token:
                    vars_data = load_vars()
                    vars_data["flowlet_token"] = token
                    save_vars(vars_data)
                    click.echo("🔐 Saved 'flowlet_token' to .flowlet_vars.json")
                else:
                    click.echo("⚠️  No token found in response.")
            except Exception as e:
                click.echo(f"❌ Failed to parse login response: {e}")
        else:
            click.echo(f"⚠️  Server error: {response.status_code} - {response.text}")

    except requests.RequestException as e:
        click.echo(f"❌ Failed to login user: {e}")


def delete_one(entity: str, query: dict):
    """Delete a document by name."""
    response = requests.post(
        f"{SERVER_URL}/delete-one/{entity}",
        json={"query": query},
        timeout=5,
    )
    if not response.ok:
        raise Exception(f"{response.status_code}: {response.text}")
