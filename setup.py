from setuptools import setup

setup(
    name="flowlet",  # or whatever you call it
    version="0.1.0",
    py_modules=["reqcli"],  # if your main file is reqcli.py
    install_requires=["click"],
    entry_points={
        "console_scripts": [
            "flo=main:cli",  # flowlet is the CLI command
        ],
    },
)
