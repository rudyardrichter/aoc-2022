fn mix(ns: &[isize], indices: &mut Vec<usize>) {
    for (i, &n) in ns.iter().enumerate() {
        let j = indices.iter().position(|&n| n == i).unwrap();
        indices.remove(j);
        indices.insert(
            (j as isize + n).rem_euclid(ns.len() as isize - 1) as usize,
            i,
        );
    }
}

fn answer_from_decrypted(ns: &[isize], indices: &[usize]) -> isize {
    let zero = indices
        .iter()
        .position(|&i| i == ns.iter().position(|&n| n == 0).unwrap())
        .unwrap();
    let l = ns.len();
    let (a, b, c) = ((zero + 1000) % l, (zero + 2000) % l, (zero + 3000) % l);
    ns[indices[a]] + ns[indices[b]] + ns[indices[c]]
}

#[aoc_generator(day20)]
pub fn get_input(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn part_1(ns: &Vec<isize>) -> isize {
    let mut indices: Vec<usize> = (0..ns.len()).collect();
    mix(&ns, &mut indices);
    answer_from_decrypted(&ns, &indices)
}

#[aoc(day20, part2)]
pub fn part_2(ns: &[isize]) -> isize {
    let ns = ns.iter().map(|&n| n * 811589153).collect::<Vec<_>>();
    let mut indices: Vec<usize> = (0..ns.len()).collect();
    (0..10).for_each(|_| mix(&ns, &mut indices));
    answer_from_decrypted(&ns, &indices)
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
