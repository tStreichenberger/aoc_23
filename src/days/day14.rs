use crate::prelude::*;
use std::collections::HashMap;

pub struct Day14;
impl Day for Day14 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<Lens>()
            .unwrap()
            .roll(Direction::North)
            .load()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        let mut lens: Lens = input.parse().unwrap();

        lens.cache();

        while lens.cycle_num < 1_000_000_000 {
            if let Some(start_of_cycle) = lens.cycle().cache() {
                let cycle_len = lens.cycle_num - start_of_cycle;
                let remaining_cycles = 1_000_000_000 - lens.cycle_num;
                let remainder = remaining_cycles % cycle_len;
                for _ in 0..remainder {
                    lens.cycle();
                }
                break;
            }
        }

        lens.load().to_string()
    }
}

struct Lens {
    grid: Grid<LensPiece>,
    cache: HashMap<Grid<LensPiece>, usize>,
    cycle_num: usize,
}

impl Lens {
    fn cycle(&mut self) -> &mut Self {
        self.cycle_num += 1;
        self.roll(Direction::North)
            .roll(Direction::West)
            .roll(Direction::South)
            .roll(Direction::East)
    }

    fn roll(&mut self, dir: Direction) -> &mut Self {
        let new_axis = match dir {
            Direction::North => Self::roll_on_axis(self.grid.cols()),
            Direction::East => Self::roll_on_axis(self.grid.rows().map(|row| row.iter().rev())),
            Direction::South => Self::roll_on_axis(self.grid.cols().map(|col| col.rev())),
            Direction::West => Self::roll_on_axis(self.grid.rows().map(|vec| vec.iter())),
        };

        for (i, ax) in new_axis.into_iter().enumerate() {
            match dir {
                Direction::North => self.grid.set_col(i, ax.into_iter()),
                Direction::East => self.grid.set_row(i, ax.into_iter().rev()),
                Direction::South => self.grid.set_col(i, ax.into_iter().rev()),
                Direction::West => self.grid.set_row(i, ax.into_iter()),
            }
        }

        self
    }

    fn roll_on_axis<'a>(
        axis: impl Iterator<Item = impl Iterator<Item = &'a LensPiece>>,
    ) -> Vec<Vec<LensPiece>> {
        axis.map(|ax| {
            ax.group_by(|piece| matches!(piece, LensPiece::SquareRock))
                .into_iter()
                .flat_map(|(_, i)| i.sorted().copied())
                .collect()
        })
        .collect()
    }

    fn load(&self) -> usize {
        self.grid.rows().rev().enumerate().sum_by(|(i, row)| {
            row.iter()
                .filter(|piece| matches!(piece, LensPiece::RoundRock))
                .count()
                * (i + 1)
        })
    }

    /// cache the current grid state and cycle num. If the grid appeared in the cache then return the cycle num for it
    fn cache(&mut self) -> Option<usize> {
        self.cache.insert(self.grid.clone(), self.cycle_num)
    }
}

impl std::fmt::Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl FromStr for Lens {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: s.parse()?,
            cache: HashMap::new(),
            cycle_num: 0,
        })
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum LensPiece {
    RoundRock,
    SquareRock,
    Lens,
}

impl From<char> for LensPiece {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::SquareRock,
            'O' => Self::RoundRock,
            '.' => Self::Lens,
            _ => panic!("Invalid lens piece {value}"),
        }
    }
}

impl From<LensPiece> for char {
    fn from(value: LensPiece) -> Self {
        match value {
            LensPiece::Lens => '.',
            LensPiece::RoundRock => 'O',
            LensPiece::SquareRock => '#',
        }
    }
}

impl std::fmt::Display for LensPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
