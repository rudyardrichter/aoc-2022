from __future__ import annotations

import heapq

from aoc_2022.utils import input_for_day


class Elf:
    def __init__(self, foods: list[int]) -> None:
        self.total = sum(foods)

    def __lt__(self, other: Elf) -> bool:
        return self.total > other.total


class ElfHeap:
    def __init__(self, data: str) -> None:
        self.elves = [
            Elf(list(map(int, chunk.splitlines()))) for chunk in data.split("\n\n")
        ]
        heapq.heapify(self.elves)


def part_1(data: str) -> int:
    # Without a heap:
    # max = 0
    # for chunk in data.split("\n\n"):
    #     elf = sum(map(int, chunk.splitlines()))
    #     if elf > max:
    #         max = elf
    # return max
    return ElfHeap(data).elves[0].total


def part_2(data: str) -> int:
    return sum(elf.total for elf in ElfHeap(data).elves[:3])


def main():
    data = input_for_day(1)
    print(part_1(data))
    print(part_2(data))


if __name__ == "__main__":
    main()
