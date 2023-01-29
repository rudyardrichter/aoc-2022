import numpy as np
import scipy

from aoc_2022.utils import input_for_day


def part_1(space: np.ndarray) -> int:
    surface_kernel = -scipy.ndimage.generate_binary_structure(3, 1).astype(np.uint8)
    surface_kernel[1, 1, 1] = 6
    return scipy.signal.convolve(space, surface_kernel, mode="same")[space].sum()


def part_2(space: np.ndarray) -> int:
    """
    scipy has a neat ``binary_fill_holes`` function, but let's re-implement.
    """
    space = space.copy()
    mask = np.logical_not(space)
    structure = scipy.ndimage.generate_binary_structure(3, 1).astype(np.uint8)
    scipy.ndimage.binary_dilation(
        np.zeros_like(space),
        structure=structure,
        iterations=-1,
        mask=mask,
        output=space,
        border_value=1,
    )
    np.logical_not(space, space)  # in-place
    return part_1(space)


def main():
    data = np.array(
        [tuple(map(int, line.split(","))) for line in input_for_day(18).split()]
    )
    space = np.zeros(data.max(axis=0) + 1, dtype=bool)
    space[tuple(data.T)] = True
    print(part_1(space))
    print(part_2(space))


if __name__ == "__main__":
    main()
