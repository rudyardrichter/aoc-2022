from pathlib import Path


def input_for_day(n: int) -> str:
    inputs = Path(__file__).parent.parent.parent.parent / "inputs"
    with open(inputs / f"day_{n:02}") as f:
        return f.read()
