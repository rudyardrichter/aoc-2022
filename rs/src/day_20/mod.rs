#[aoc_generator(day20)]
pub fn get_input(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn part_1(ns: &Vec<isize>) -> isize {
    let l = ns.len();
    let mut indices: Vec<usize> = (0..l).collect();
    for (i, &n) in ns.iter().enumerate() {
        let j = indices.iter().position(|&n| n == i).unwrap();
        indices.remove(j);
        indices.insert((j as isize + n).rem_euclid(l as isize - 1) as usize, i);
    }
    let z = indices
        .iter()
        .position(|&i| i == ns.iter().position(|&n| n == 0).unwrap())
        .unwrap();
    let (a, b, c) = ((z + 1000) % l, (z + 2000) % l, (z + 3000) % l);
    ns[indices[a]] + ns[indices[b]] + ns[indices[c]]
}

#[aoc(day20, part2)]
pub fn part_2(ns: &[isize]) -> isize {
    let ns = ns.iter().map(|&n| n * 811589153).collect::<Vec<_>>();
    let l = ns.len();
    let mut indices: Vec<usize> = (0..l).collect();
    for _ in 0..10 {
        for (i, &n) in ns.iter().enumerate() {
            let j = indices.iter().position(|&n| n == i).unwrap();
            indices.remove(j);
            indices.insert((j as isize + n).rem_euclid(l as isize - 1) as usize, i);
        }
    }
    let z = indices
        .iter()
        .position(|&i| i == ns.iter().position(|&n| n == 0).unwrap())
        .unwrap();
    let (a, b, c) = ((z + 1000) % l, (z + 2000) % l, (z + 3000) % l);
    ns[indices[a]] + ns[indices[b]] + ns[indices[c]]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 1623178306);
    }
}
