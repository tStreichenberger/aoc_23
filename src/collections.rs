use std::{
    convert::Infallible,
    default::Default,
    str::FromStr,
};

pub struct DigitSet {
    set: Vec<bool>,
}

impl DigitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_size(num_digits: u32) -> Self {
        let len = 10_usize.pow(num_digits);
        Self {
            set: vec![false; len],
        }
    }

    pub fn reallocate(&mut self, num_digits: u32) {
        let current_len = self.set.len();
        let new_len = 10_usize.pow(num_digits);
        self.set.extend((current_len..new_len).map(|_| false))
    }

    pub fn contains(&self, num: &usize) -> bool {
        *self.set.get(*num).unwrap_or(&false)
    }

    // adds num to DigitSet. Returns true if the element was not already present
    pub fn insert(&mut self, num: usize) -> bool {
        self.set
            .get_mut(num)
            .map(|b| {
                let r = !*b;
                *b = true;
                r
            })
            .unwrap_or_else(|| {
                let new_size = f64::log10(num as f64).ceil() as u32;
                self.reallocate(new_size);
                self.insert(num)
            })
    }
}

impl FromIterator<usize> for DigitSet {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut set = DigitSet::new();
        iter.into_iter().for_each(|num| {
            set.insert(num);
        });
        set
    }
}

impl Default for DigitSet {
    fn default() -> Self {
        Self::new_with_size(2)
    }
}

pub type Index = (usize, usize);

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s
                .lines()
                .map(|line| line.chars().map(T::from).collect())
                .collect(),
        })
    }
}

impl<T> std::ops::Index<Index> for Grid<T> {
    type Output = T;
    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> Grid<T> {
    pub fn get(&self, i: Index) -> Option<&T> {
        self.data.get(i.0).and_then(|row| row.get(i.1))
    }

    pub fn get_mut(&mut self, i: Index) -> Option<&mut T> {
        self.data.get_mut(i.0).and_then(|row| row.get_mut(i.1))
    }

    pub fn cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &T> + Clone>
           + std::iter::DoubleEndedIterator
           + Clone
           + std::iter::ExactSizeIterator {
        (0..self.data[0].len()).map(|i| self.data.iter().map(move |row| &row[i]))
    }

    pub fn set_col(&mut self, index: usize, new_vals: impl Iterator<Item = T>) {
        self.data
            .iter_mut()
            .zip(new_vals)
            .for_each(|(mut_row, col_val)| mut_row[index] = col_val)
    }

    pub fn rows(&self) -> std::slice::Iter<'_, Vec<T>> {
        self.data.iter()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'g, T> IntoIterator for &'g Grid<T> {
    type Item = &'g Vec<T>;
    type IntoIter = std::slice::Iter<'g, Vec<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'g, T> IntoIterator for &'g mut Grid<T> {
    type Item = &'g mut Vec<T>;
    type IntoIter = std::slice::IterMut<'g, Vec<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self { data: value }
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for i in row {
                write!(f, "{i}").unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
