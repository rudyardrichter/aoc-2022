use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::grid::Grid;

#[derive(Debug, Eq, Ord, PartialEq)]
struct HeapItem {
    i: usize, // grid index
    p: usize, // priority
    v: u8,    // value
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &HeapItem) -> Option<Ordering> {
        // backwards since we want a min heap
        other.p.partial_cmp(&self.p)
    }
}

fn hike(grid: &Grid<u8>, any_start: bool) -> usize {
    let mut grid = grid.clone();
    let mut origin_opt: Option<usize> = None;
    let mut destination_opt: Option<usize> = None;
    let mut q: BinaryHeap<HeapItem> = BinaryHeap::new();
    let mut distance: HashMap<usize, Option<usize>> = HashMap::new();
    for (i, &v) in grid.items.iter().enumerate() {
        if v == 'E' as u8 {
            q.push(HeapItem {
                i,
                p: usize::MAX,
                v: 'z' as u8,
            });
            origin_opt = Some(i);
            distance.insert(i, Some(0));
        } else {
            distance.insert(i, None);
        }
        if v == 'S' as u8 {
            destination_opt = Some(i);
        }
    }
    let origin = origin_opt.unwrap_or_else(|| panic!("no end found"));
    let destination = destination_opt.unwrap_or_else(|| panic!("no start found"));
    grid[destination] = 'a' as u8;
    grid[origin] = 'z' as u8;
    while let Some(item) = q.pop() {
        for (j, v) in grid
            .neighbors(item.i)
            .into_iter()
            .filter(|(_, v)| *v + 1 >= item.v)
        {
            let d_new = distance[&item.i].unwrap() + 1;
            if (any_start && v == 'a' as u8) || (!any_start && j == destination) {
                return d_new;
            }
            if distance[&j].map_or(true, |d| d > d_new) {
                distance.insert(j, Some(d_new));
                q.push(HeapItem { i: j, p: d_new, v });
            }
        }
    }
    distance[&destination].unwrap()
}

#[aoc_generator(day12)]
pub fn get_input(input: &str) -> Grid<u8> {
    Grid::try_from(input).unwrap()
}

#[aoc(day12, part1)]
pub fn part_1(grid: &Grid<u8>) -> usize {
    hike(grid, false)
}

#[aoc(day12, part2)]
pub fn part_2(grid: &Grid<u8>) -> usize {
    hike(grid, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_12.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 29);
    }
}
