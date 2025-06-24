import requests
import click


def insert(entity: str, data: dict):
    """Send a command to the remote server."""
    try:
        response = requests.post(
            f"http://localhost:8080/insert-one/{entity}",
            json=data,
            timeout=5,
        )
        if response.ok:
            click.echo("☁️  Inserted document.")
        else:
            click.echo(f"⚠️  Server error: {response.status_code} - {response.text}")
    except requests.RequestException as e:
        click.echo(f"❌ Failed to sync with remote server: {e}")


def find_many(entity: str) -> list[dict]:
    try:
        response = requests.post(
            f"http://localhost:8080/find-many/{entity}",
            json={},
            timeout=5,
        )
        if response.ok:
            return response.json().get("data", [])
        else:
            click.echo(f"⚠️  Server error: {response.status_code} - {response.text}")
    except requests.RequestException as e:
        click.echo(f"❌ Failed to sync with remote server: {e}")
    return []
