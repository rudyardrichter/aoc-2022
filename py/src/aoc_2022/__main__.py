import importlib
import os
import pathlib
import re

from .utils.input import ensure_input_exists


def most_recent_day() -> int:
    return max(
        int(re.findall(r"\d+", fp)[0])
        for fp in os.listdir(pathlib.Path(__file__).parent / "days")
        if re.match("day", fp)
    )


def run_day(n: int) -> None:
    importlib.import_module(f"aoc_2022.days.day_{n:02}").main()


def main():
    most_recent_n = most_recent_day()
    ensure_input_exists(most_recent_n)
    run_day(most_recent_n)
