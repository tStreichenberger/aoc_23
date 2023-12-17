use crate::prelude::*;

pub struct Day14;
impl Day for Day14 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<Lens>()
            .unwrap()
            .roll_north()
            .load()
            .to_string()
    }
}

struct Lens {
    grid: Grid<LensPiece>,
}

impl Lens {
    fn roll_north(&mut self) -> &mut Self {
        let new_cols = self
            .grid
            .cols()
            .map(|col| {
                col.group_by(|piece| matches!(piece, LensPiece::SquareRock))
                    .into_iter()
                    .flat_map(|(_, i)| i.sorted().copied())
                    .collect_vec()
            })
            .collect_vec();

        for (i, new_col) in new_cols.into_iter().enumerate() {
            self.grid.set_col(i, new_col.into_iter())
        }

        self
    }

    fn load(&self) -> usize {
        self.grid.rows().rev().enumerate().sum_by(|(i, row)| {
            row.iter()
                .filter(|piece| matches!(piece, LensPiece::RoundRock))
                .count()
                * (i + 1)
        })
    }
}

impl FromStr for Lens {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
