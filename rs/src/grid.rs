use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

/// Represent a two-dimensional grid in a flat structure.
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

impl<T: Clone> Grid<T> {
    pub fn neighbors(&self, i: usize) -> impl std::iter::IntoIterator<Item = (usize, T)> {
        self.neighbor_ixs(i)
            .into_iter()
            .map(move |j| (j, self.items[j].clone()))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn neighbor_ixs(&self, i: usize) -> impl std::iter::IntoIterator<Item = usize> {
        let mut result: Vec<usize> = Vec::new();
        let x = i % self.w;
        if i >= self.w {
            result.push(i - self.w); // ↑
        }
        if x > 0 {
            result.push(i - 1); // ←
        }
        if x < self.w - 1 {
            result.push(i + 1); // →
        }
        if i < self.items.len() - self.w {
            result.push(i + self.w); // ↓
        }
        result.into_iter()
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
            .map(|l| l.chars().map(T::try_from))
            .flatten()
            .collect::<Result<Vec<T>, _>>()
            .map_err(|e| format!("{:?}", e))?;
        Ok(Grid { items, w })
    }
}
