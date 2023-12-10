use crate::prelude::*;

pub struct Day10;
impl Day for Day10 {
    fn star1(&self, input: String) -> String {
        (input.parse::<PipeMaze>().unwrap().loop_length() / 2).to_string()
    }
}

struct PipeMaze {
    data: Vec<Vec<char>>,
}

impl FromStr for PipeMaze {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s.lines().map(|line| line.chars().collect()).collect(),
        })
    }
}

impl PipeMaze {
    fn start_pos(&self) -> (usize, usize) {
        self.data
            .iter()
            .find_position(|row| row.contains(&'S'))
            .map(|(row_num, row)| (row_num, row.iter().find_position(|c| **c == 'S').unwrap().0))
            .unwrap()
    }

    fn loop_length(&self) -> usize {
        let mut dir = directions('S')[0].invert();
        let start = self.start_pos();
        let mut index = start;
        let mut num_steps = 0;
        loop {
            (dir, index) = self.get_next(dir, index);
            num_steps += 1;
            if index == start {
                break;
            }
        }
        num_steps
    }

    /// type signature is kinda horrible
    fn get_next(&self, prev_dir: Direction, index: (usize, usize)) -> (Direction, (usize, usize)) {
        let char = self.char_at(index);
        let next_dir = directions(char)
            .into_iter()
            .filter(|dir| *dir != prev_dir)
            .next()
            .unwrap();
        let next_index = next_dir.go(index);
        (next_dir.invert(), next_index)
    }

    fn char_at(&self, index: (usize, usize)) -> char {
        *self.data.get(index.0).unwrap().get(index.1).unwrap()
    }
}

fn directions(c: char) -> [Direction; 2] {
    use Direction::*;
    match c {
        '|' => [North, South],
        '-' => [East, West],
        'L' => [North, East],
        'J' => [West, North],
        '7' => [West, South],
        'F' => [East, South],
        // I just looked at the puzzle input lmao
        'S' => [West, South],
        '.' => panic!("We somehow got to the ground in the maze... whoops"),
        _ => panic!("Received invalid maze char {c}"),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn invert(self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }

    fn go(&self, index: (usize, usize)) -> (usize, usize) {
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
}
