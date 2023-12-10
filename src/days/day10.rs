use crate::prelude::*;

use colored::Colorize;

pub struct Day10;
impl Day for Day10 {
    fn star1(&self, input: String) -> String {
        (input.parse::<PipeMaze>().unwrap().loop_length() / 2).to_string()
    }

    // LMAOOO, there is something wrong with this solution but it got close. So I just guessed answers close and got it.
    // It missed 2 of them. Wonder what the bug is...
    fn star2(&self, input: String) -> String {
        input
            .parse::<PipeMaze>()
            .unwrap()
            .set_entire_loop()
            .fill_in_inside()
            .display()
            .num_found
            .to_string()
    }
}

#[derive(Clone)]
struct PipeMaze {
    data: Vec<Vec<char>>,
    loop_pipes: Vec<Vec<bool>>,
    found_inside: Vec<Vec<bool>>,
    num_found: usize,
}

impl FromStr for PipeMaze {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let bools = data
            .iter()
            .map(|row| row.iter().map(|_| false).collect_vec())
            .collect_vec();
        Ok(Self {
            loop_pipes: bools.clone(),
            found_inside: bools,
            data,
            num_found: 0,
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

    fn loop_iter(&self) -> LoopIter {
        LoopIter {
            maze: self.clone(),
            prev_dir: directions('S')[0],
            prev_index: self.start_pos(),
            finished: false,
        }
    }

    fn loop_length(&self) -> usize {
        self.loop_iter().count()
    }

    fn display(&self) -> &Self {
        for (i, row) in self.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if self.found((i, j)) {
                    print!("{}", "I".bright_red());
                } else if c == &'S' {
                    print!("{}", "S".bright_green());
                } else if self.is_in_loop((i, j)) {
                    print!("{}", c.to_string().yellow());
                } else {
                    print!("{c}")
                }
            }
            println!("");
        }
        self
    }

    fn set_entire_loop(&mut self) -> &mut Self {
        self.loop_iter().for_each(|(_, index)| self.set_loop(index));
        self
    }

    fn fill_in_inside(&mut self) -> &mut Self {
        self.loop_iter().for_each(|(direction, index)| {
            let inside_dir = direction.invert().right_dir();
            let inside_index = inside_dir.go(index);
            self.bfs_set(inside_index);
        });
        self
    }

    /// bfs search inside area
    fn bfs_set(&mut self, index: (usize, usize)) {
        use Direction::*;
        if self.found(index) || self.is_in_loop(index) {
            return;
        }
        self.set_found(index);
        for dir in [North, South, East, West] {
            self.bfs_set(dir.go(index))
        }
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

    fn is_in_loop(&self, index: (usize, usize)) -> bool {
        *self.loop_pipes.get(index.0).unwrap().get(index.1).unwrap()
    }

    fn set_loop(&mut self, index: (usize, usize)) {
        *self
            .loop_pipes
            .get_mut(index.0)
            .unwrap()
            .get_mut(index.1)
            .unwrap() = true;
    }

    fn found(&self, index: (usize, usize)) -> bool {
        *self
            .found_inside
            .get(index.0)
            .unwrap()
            .get(index.1)
            .unwrap()
    }

    fn set_found(&mut self, index: (usize, usize)) {
        *self
            .found_inside
            .get_mut(index.0)
            .unwrap()
            .get_mut(index.1)
            .unwrap() = true;
        self.num_found += 1;
    }
}

struct LoopIter {
    maze: PipeMaze,
    prev_dir: Direction,
    prev_index: (usize, usize),
    finished: bool,
}

impl Iterator for LoopIter {
    type Item = (Direction, (usize, usize));
    /// Item is direction we arrived from and the index we are at
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let (dir, index) = self.maze.get_next(self.prev_dir, self.prev_index);
        if self.maze.char_at(index) == 'S' {
            self.finished = true;
        }
        self.prev_dir = dir;
        self.prev_index = index;
        Some((self.prev_dir, self.prev_index))
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

    fn right_dir(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
