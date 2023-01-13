use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use num::Integer;

/// Represent a two-dimensional grid in a flat structure.
#[derive(Debug)]
pub struct Grid<T> {
    pub items: Vec<T>,
    pub w: usize,
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Grid {
            items: self.items.clone(),
            w: self.w,
        }
    }
}

impl<T, Idx: SliceIndex<[T]>> Index<Idx> for Grid<T> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.items.index(index)
    }
}

impl<T, Idx: SliceIndex<[T]>> IndexMut<Idx> for Grid<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.items.index_mut(index)
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(w: usize) -> Self {
        Grid {
            items: vec![T::default(); w * w],
            w,
        }
    }
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
    pub fn xy_to_i(&self, (x, y): (usize, usize)) -> usize {
        x + y * self.w
    }

    pub fn i_to_xy(&self, i: usize) -> (usize, usize) {
        let (y, x) = i.div_rem(&self.w);
        (x, y)
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.items.get(i)
    }

    pub fn get_xy(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.items.get(self.xy_to_i((x, y)))
    }

    pub fn neighbors(&self, i: usize) -> Vec<(usize, T)> {
        self.neighbor_ixs(i)
            .into_iter()
            .map(move |j| (j, self.items[j].clone()))
            .collect::<Vec<_>>()
    }

    pub fn neighbor_ixs(&self, i: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let x = i % self.w;
        if i >= self.w {
            result.push(i - self.w); // ↑
        }
        if x < self.w - 1 {
            result.push(i + 1); // →
        }
        if i < self.items.len() - self.w {
            result.push(i + self.w); // ↓
        }
        if x > 0 {
            result.push(i - 1); // ←
        }
        result
    }

    pub fn neighbors_diagonal(&self, i: usize) -> Vec<(usize, T)> {
        self.neighbor_ixs_diagonal(i)
            .into_iter()
            .map(move |j| (j, self.items[j].clone()))
            .collect::<Vec<_>>()
    }

    pub fn neighbor_ixs_diagonal(&self, i: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        if i > self.w {
            result.push(i - self.w - 1); // ↖
        }
        if i > self.w - 1 {
            result.push(i - self.w + 1); // ↗
        }
        if i < self.items.len() - self.w {
            result.push(i + self.w - 1); // ↙
        }
        if i < self.items.len() - self.w - 1 {
            result.push(i + self.w + 1); // ↘
        }
        result
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.items.into_iter()
    }
}

impl<T: TryFrom<char>> TryFrom<&str> for Grid<T>
where
    <T as TryFrom<char>>::Error: Debug,
{
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let w = s.lines().next().ok_or("empty input")?.len();
        let items = s
            .lines()
            .flat_map(|l| l.chars().map(T::try_from))
            .collect::<Result<Vec<T>, _>>()
            .map_err(|e| format!("{:?}", e))?;
        Ok(Grid { items, w })
    }
}
