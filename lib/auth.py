import click
from lib.api import register, login
from lib.utils import remove_var


@click.group(name="auth")
def auth_group():
    """Register and login to sync with cloud."""
    pass


@auth_group.command(name="register")
@click.option("--email", prompt=True)
@click.option("--password", prompt=True, hide_input=True, confirmation_prompt=True)
def register_user(email, password):
    """Register a new user on the remote server."""
    register(email, password)


@auth_group.command(name="login")
@click.option("--email", prompt=True)
@click.option("--password", prompt=True, hide_input=True)
def login_user(email, password):
    """Login user on the remote server."""
    login(email, password)


@auth_group.command(name="logout")
def logout_user():
    """Sign the current user out."""
    # TODO: Implement Logout on Server
    # logout()
    remove_var("flowlet_token")
    click.secho("👋 Logged out successfully. See you soon!", fg="cyan", bold=True)
