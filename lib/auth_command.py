import click
from lib.api import register, login
from lib.utils import remove_var


@click.command(name="register")
@click.option("--email", prompt=True)
@click.option("--password", prompt=True, hide_input=True, confirmation_prompt=True)
def register_user(email, password):
    """Register a new user on the remote server."""
    register(email, password)


@click.command(name="login")
@click.option("--email", prompt=True)
@click.option("--password", prompt=True, hide_input=True)
def login_user(email, password):
    """Login user on the remote server."""
    login(email, password)

@click.command(name="logout")
def logout_user():
    """Sign the current user out."""
    # TODO: Implement Logout on Server
    # logout()
    remove_var("flowlet_token")
    click.secho("👋 Logged out successfully. See you soon!", fg="cyan", bold=True)

