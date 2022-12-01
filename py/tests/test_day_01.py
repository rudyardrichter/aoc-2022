from aoc_2022.days.day_01 import part_1, part_2

INPUT = """\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000\
"""


def test_part_1():
    assert part_1(INPUT) == 24000


def test_part_2():
    assert part_2(INPUT) == 45000
