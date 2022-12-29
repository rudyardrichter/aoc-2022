use std::collections::HashSet;

use sscanf::scanf;

#[aoc_generator(day18)]
pub fn get_input(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .lines()
        .map(|l| scanf!(l, "{isize},{isize},{isize}").unwrap())
        .collect()
}

fn neighbors((x, y, z): (isize, isize, isize)) -> [(isize, isize, isize); 6] {
    // ok if these overflow, since we're just checking membership
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

#[aoc(day18, part1)]
pub fn part_1(cubes: &HashSet<(isize, isize, isize)>) -> usize {
    cubes
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|p| !cubes.contains(p))
        .count()
}

#[aoc(day18, part2)]
pub fn part_2(cubes: &HashSet<(isize, isize, isize)>) -> usize {
    let mut visited: HashSet<(isize, isize, isize)> = HashSet::new();
    let (max_x, max_y, max_z, min_x, min_y, min_z) = cubes.iter().fold(
        (0, 0, 0, isize::MAX, isize::MAX, isize::MAX),
        |(max_x, max_y, max_z, min_x, min_y, min_z), &(x, y, z)| {
            (
                max_x.max(x + 1),
                max_y.max(y + 1),
                max_z.max(z + 1),
                min_x.min(x - 1),
                min_y.min(y - 1),
                min_z.min(z - 1),
            )
        },
    );
    let mut q = vec![(min_x, min_y, min_z)];
    while let Some(p) = q.pop() {
        let next: Vec<(isize, isize, isize)> = neighbors(p)
            .iter()
            .filter(|p| {
                min_x <= p.0
                    && p.0 <= max_x
                    && min_y <= p.1
                    && p.1 <= max_y
                    && min_z <= p.2
                    && p.2 <= max_z
                    && !visited.contains(p)
                    && !cubes.contains(p)
            })
            .copied()
            .collect();
        visited.extend(next.clone());
        q.extend(next);
    }
    cubes
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|p| visited.contains(p))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_18.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 58);
    }
}
