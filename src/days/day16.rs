use crate::prelude::*;

pub struct Day16;
impl Day for Day16 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<MirrorMaze>()
            .unwrap()
            .traverse((0, 0), Direction::West)
            .num_energized()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        let maze: MirrorMaze = input.parse().unwrap();
        // from top
        (0..maze.grid.num_cols())
            .map(|j| {
                maze.clone()
                    .traverse((0, j), Direction::North)
                    .num_energized()
            })
            // from bottom
            .chain((0..maze.grid.num_cols()).map(|j| {
                maze.clone()
                    .traverse((maze.grid.num_rows(), j), Direction::South)
                    .num_energized()
            }))
            // from left
            .chain((0..maze.grid.num_rows()).map(|i| {
                maze.clone()
                    .traverse((i, 0), Direction::West)
                    .num_energized()
            }))
            // from right
            .chain((0..maze.grid.num_rows()).map(|i| {
                maze.clone()
                    .traverse((i, maze.grid.num_cols()), Direction::East)
                    .num_energized()
            }))
            .max()
            .unwrap()
            .to_string()
    }
}

#[derive(Clone)]
struct MirrorMaze {
    grid: Grid<Mirror>,
    /// for each grid square put directions we have gone to on the square
    traversed: Grid<Vec<Direction>>,
}

impl FromStr for MirrorMaze {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Mirror> = s.parse()?;
        let traversed = grid
            .rows()
            .map(|row| row.iter().map(|_| Vec::new()).collect())
            .collect();
        Ok(Self { grid, traversed })
    }
}

impl MirrorMaze {
    fn num_energized(&self) -> usize {
        self.traversed
            .rows()
            .sum_by(|row| row.iter().filter(|square| !square.is_empty()).count())
    }

    fn traverse(&mut self, i: Index, coming_from: Direction) -> &mut Self {
        let Some(mirror) = self.grid.get(i) else {
            return self;
        };
        for next_dir in mirror.next_dir(coming_from) {
            if self.traversed.get(i).unwrap().contains(&next_dir) {
                continue;
            }
            self.traversed.get_mut(i).unwrap().push(next_dir);
            let next_index = next_dir.go(i);
            self.traverse(next_index, next_dir.invert());
        }
        self
    }

    #[allow(unused)]
    fn print_energized(&self) -> &Self {
        self.traversed
            .rows()
            .map(|row| {
                row.iter()
                    .map(|square| if square.is_empty() { '.' } else { '#' })
                    .collect()
            })
            .collect::<Grid<_>>()
            .display();
        self
    }
}

#[derive(Clone)]
enum Mirror {
    SplitToVertical,
    SplitToHorizontal,
    LeftToUp,
    LeftToDown,
    None,
}

impl Mirror {
    fn next_dir(&self, coming_from: Direction) -> Vec<Direction> {
        match (self, coming_from) {
            (Self::SplitToHorizontal, Direction::East | Direction::West) => {
                vec![coming_from.invert()]
            }
            (Self::SplitToHorizontal, Direction::North | Direction::South) => {
                vec![Direction::East, Direction::West]
            }
            (Self::SplitToVertical, Direction::North | Direction::South) => {
                vec![coming_from.invert()]
            }
            (Self::SplitToVertical, Direction::East | Direction::West) => {
                vec![Direction::North, Direction::South]
            }
            (Self::None, _) => vec![coming_from.invert()],
            // '\'
            (Self::LeftToDown, Direction::North) => vec![Direction::East],
            (Self::LeftToDown, Direction::East) => vec![Direction::North],
            (Self::LeftToDown, Direction::South) => vec![Direction::West],
            (Self::LeftToDown, Direction::West) => vec![Direction::South],
            // '/'
            (Self::LeftToUp, Direction::North) => vec![Direction::West],
            (Self::LeftToUp, Direction::East) => vec![Direction::South],
            (Self::LeftToUp, Direction::South) => vec![Direction::East],
            (Self::LeftToUp, Direction::West) => vec![Direction::North],
        }
    }
}

impl From<char> for Mirror {
    fn from(c: char) -> Self {
        match c {
            '.' => Mirror::None,
            '\\' => Mirror::LeftToDown,
            '/' => Mirror::LeftToUp,
            '-' => Mirror::SplitToHorizontal,
            '|' => Mirror::SplitToVertical,
            _ => panic!("Recieve invalid Mirror char {c}"),
        }
    }
}
