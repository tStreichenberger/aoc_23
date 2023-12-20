#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn invert(self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }

    pub fn go(&self, index: (usize, usize)) -> (usize, usize) {
        use Direction::*;
        let diff = match self {
            North => (-1, 0),
            South => (1, 0),
            West => (0, -1),
            East => (0, 1),
        };
        (
            (index.0 as isize + diff.0) as usize,
            (index.1 as isize + diff.1) as usize,
        )
    }

    pub fn sgo(&self, index: SIndex) -> SIndex {
        use Direction::*;
        let diff = match self {
            North => (-1, 0),
            South => (1, 0),
            West => (0, -1),
            East => (0, 1),
        };
        ((index.0 + diff.0), (index.1 + diff.1))
    }

    pub fn right_dir(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn left_dir(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::North,
            'L' => Direction::West,
            'R' => Direction::East,
            'D' => Direction::South,
            _ => panic!("Invalid direction char: {c}"),
        }
    }
}

pub type SIndex = (i64, i64);
