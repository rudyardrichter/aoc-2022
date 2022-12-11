import os
import pathlib

TEMPLATE = """\
#[aoc_generator(day{day})]
pub fn get_input(input: &str) -> () {{
    ()
}}

#[aoc(day{day}, part1)]
pub fn part_1(_: &()) -> usize {{
    0
}}

#[aoc(day{day}, part2)]
pub fn part_2(_: &()) -> usize {{
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_{day:02}.txt");

    #[test]
    fn test_part_1() {{}}

    #[test]
    fn test_part_2() {{}}
}}\
"""


def main() -> None:
    next_day = (
        max(
            int(day_dir[-2:])
            for day_dir in os.listdir(
                pathlib.Path(os.path.dirname(__file__)).parent / "src"
            )
            if day_dir.startswith("day")
        )
        + 1
    )
    new_dir = pathlib.Path("src") / f"day_{next_day:02}"
    os.mkdir(new_dir)
    with open(new_dir / "mod.rs", "w") as f:
        f.write(TEMPLATE.format(day=next_day))


if __name__ == "__main__":
    main()
