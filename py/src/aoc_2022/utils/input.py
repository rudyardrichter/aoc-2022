"""
TODO: would be nicer to use importlib to find files
"""
import json
import os
import pathlib

import requests


def load_creds() -> dict[str, str]:
    """
    Requires aoc_creds.json file with contents like {"session": "..."}
    """
    filepath = str(
        pathlib.Path(__file__).parent.parent.parent.parent / "aoc_creds.json"
    )
    if not os.path.exists(filepath):
        raise EnvironmentError("aoc_creds.json file required; save session token there")
    with open(filepath) as f:
        return json.load(f)


def ensure_input_exists(day: int) -> None:
    filepath = (
        pathlib.Path(__file__).parent.parent.parent.parent / "inputs" / f"day_{day:02}"
    )
    if not os.path.exists(filepath):
        session_cookie = load_creds()["session"]
        headers = {"Cookie": f"session={session_cookie}"}
        response = requests.get(
            f"https://adventofcode.com/2022/day/{day}/input",
            headers=headers,
        )
        response.raise_for_status()
        with open(filepath, "w") as f:
            f.write(response.text)


def input_for_day(day: int) -> str:
    ensure_input_exists(day)
    inputs = pathlib.Path(__file__).parent.parent.parent.parent / "inputs"
    with open(inputs / f"day_{day:02}") as f:
        return f.read()
