use std::ops::Index;

use crate::prelude::*;

pub struct Day03;
impl Day for Day03 {
    fn star1(&self, input: String) -> String {
        let mut total_parts: usize = 0;
        let mut engine: Engine = input.parse().unwrap();
        for i in 0..engine.len() {
            for j in 0..engine[i].len() {
                if engine.has_symbol_at(i, j) {
                    let parts = engine.scan_for_parts(i, j);
                    total_parts += parts.iter().sum::<usize>();
                }
            }
        }
        total_parts.to_string()
    }

    fn star2(&self, input: String) -> String {
        let mut total_ratio: usize = 0;
        let mut engine: Engine = input.parse().unwrap();
        for i in 0..engine.len() {
            for j in 0..engine[i].len() {
                if engine[i][j] == '*' {
                    total_ratio += engine.gear_ratio(i, j);
                }
            }
        }
        total_ratio.to_string()
    }
}

struct Engine {
    raw_data: Vec<Vec<char>>,
    found_parts: Vec<Vec<bool>>,
}

impl FromStr for Engine {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data = s
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let found_parts = raw_data
            .iter()
            .map(|line| line.iter().map(|_| false).collect())
            .collect();
        Ok(Self {
            raw_data,
            found_parts,
        })
    }
}

impl Engine {
    fn len(&self) -> usize {
        self.raw_data.len()
    }

    /// Takes indicies pointing to a digit of a part.
    /// returns the whole part.
    ///
    /// If the part was previously returned then this will return None
    fn get_parts(&mut self, i: usize, j: usize) -> Option<usize> {
        if self.found_parts[i][j] {
            return None;
        }
        let row = &self[i];
        let mut left_index = j;
        while let Some(true) = row.get(left_index - 1).map(|c| c.is_digit(10)) {
            left_index -= 1
        }
        let mut right_index = j;
        while let Some(true) = row.get(right_index + 1).map(|c| c.is_digit(10)) {
            right_index += 1
        }

        for j in left_index..=right_index {
            self.found_parts[i][j] = true;
        }

        Some(
            self[i][left_index..=right_index]
                .iter()
                .join("")
                .parse()
                .unwrap(),
        )
    }

    fn scan_for_parts(&mut self, i: usize, j: usize) -> Vec<usize> {
        let mut found_parts = Vec::new();
        for i in i - 1..=i + 1 {
            for j in j - 1..=j + 1 {
                if let Some(true) = self.index(i).get(j).map(|c| c.is_digit(10)) {
                    if let Some(part) = self.get_parts(i, j) {
                        found_parts.push(part)
                    }
                }
            }
        }
        found_parts
    }

    fn has_symbol_at(&self, i: usize, j: usize) -> bool {
        let c = self[i][j];
        !c.is_digit(10) && c != '.'
    }

    fn gear_ratio(&mut self, i: usize, j: usize) -> usize {
        // kinda horrible but :shrug:
        self.found_parts
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|b| *b = false));
        let parts = self.scan_for_parts(i, j);
        match parts.len() {
            2 => parts[0] * parts[1],
            _ => 0,
        }
    }
}

impl Index<usize> for Engine {
    type Output = Vec<char>;
    fn index(&self, index: usize) -> &Self::Output {
        self.raw_data.index(index)
    }
}
