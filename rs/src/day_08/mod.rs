use std::collections::HashSet;

fn visible_indices<T>(v: &Vec<T>) -> HashSet<usize>
where
    T: Ord,
{
    if v.is_empty() {
        return HashSet::new();
    }
    let mut highest_fw = &v[0];
    let mut highest_bw = &v[v.len() - 1];
    let mut results = HashSet::from([0, v.len() - 1]);
    for i in 1..v.len() {
        if &v[i] > highest_fw {
            results.insert(i);
            highest_fw = &v[i];
        }
        if &v[v.len() - i - 1] > highest_bw {
            results.insert(v.len() - i - 1);
            highest_bw = &v[v.len() - i - 1];
        }
    }
    results
}

// Index represents tree height 0–9, value represents distance from a point to the nearest tree of
// that height or greater
type DistToHeight = [usize; 10];

#[derive(Debug)]
struct Scenery {
    here: usize,
    up: DistToHeight,
    down: DistToHeight,
    left: DistToHeight,
    right: DistToHeight,
}

impl Scenery {
    fn score(&self) -> usize {
        self.up[self.here] * self.down[self.here] * self.left[self.here] * self.right[self.here]
    }
}

fn sceneries_for(vs: &Vec<Vec<usize>>) -> Vec<Vec<Scenery>> {
    if vs.is_empty() {
        return Vec::new();
    }
    let mut results: Vec<Vec<Scenery>> = vs
        .iter()
        .map(|v| {
            v.iter()
                .map(|h| Scenery {
                    here: *h,
                    up: [0; 10],
                    down: [0; 10],
                    left: [0; 10],
                    right: [0; 10],
                })
                .collect()
        })
        .collect();
    // e.g. results[i][0].left stays zeroed.
    // At each step going "right", copy the previous LEFT DistToHeight forwards, and update the
    // distance for the immediately previous tree's height.
    for i in 0..vs.len() {
        for j in 1..vs.len() {
            for h in 0..=9 {
                results[i][j].left[h] = results[i][j - 1].left[h] + 1;
            }
            (0..=vs[i][j - 1]).for_each(|h| results[i][j].left[h] = 1);
            // same going in opposite direction
            for h in 0..=9 {
                results[i][vs.len() - j - 1].right[h] = results[i][vs.len() - j].right[h] + 1;
            }
            (0..=vs[i][vs.len() - j]).for_each(|h| results[i][vs.len() - j - 1].right[h] = 1);
            // now again for up and down
            for h in 0..=9 {
                results[j][i].up[h] = results[j - 1][i].up[h] + 1;
            }
            (0..=vs[j - 1][i]).for_each(|h| results[j][i].up[h] = 1);
            for h in 0..=9 {
                results[vs.len() - j - 1][i].down[h] = results[vs.len() - j][i].down[h] + 1;
            }
            (0..=vs[vs.len() - j][i]).for_each(|h| results[vs.len() - j - 1][i].down[h] = 1)
        }
    }
    results
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return Vec::new();
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[aoc_generator(day8)]
pub fn get_input(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part_1(trees: &[Vec<usize>]) -> usize {
    // TODO: could reimplement using sceneries_for
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    visible.extend(
        trees
            .iter()
            .enumerate()
            .flat_map(|(i, line)| visible_indices(line).into_iter().map(move |j| (i, j))),
    );
    visible.extend(
        transpose(trees.to_owned())
            .iter()
            .enumerate()
            .flat_map(|(i, line)| visible_indices(line).into_iter().map(move |j| (j, i))),
    );
    visible.len()
}

#[aoc(day8, part2)]
pub fn part_2(trees: &Vec<Vec<usize>>) -> usize {
    sceneries_for(trees)
        .iter()
        .map(|v| v.iter().map(|s| s.score()).max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "30373\n25512\n65332\n33549\n35390\n";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 8);
    }
}
