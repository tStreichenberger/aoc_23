use crate::prelude::*;

pub struct Day13;
impl Day for Day13 {
    fn star1(&self, input: String) -> String {
        input
            .split("\n\n")
            .parse_each()
            .map(|m: Mirror| m.find_reflection().score())
            .sum::<usize>()
            .to_string()
    }
}

struct Mirror {
    grid: Grid<char>,
}

impl FromStr for Mirror {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = s.parse()?;

        Ok(Self { grid })
    }
}

impl Mirror {
    fn find_reflection(&self) -> Reflection {
        self.find_horizontal()
            .or_else(|| self.find_vertical())
            .expect("Did not find horizontal or vertical reflection")
    }

    fn find_horizontal(&self) -> Option<Reflection> {
        log!("____");
        let actual_reflection = self
            .grid
            .rows()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (r1, r2))| r1 == r2)
            .map(|(i, _)| {
                debug!(i);
                let going_up = self.grid.rows().take(i + 1).rev();
                let mut rows = self.grid.rows();
                for _ in 0..=i {
                    rows.next();
                }
                let going_down = rows;
                (i, (going_up, going_down))
            })
            .find(|(_, (going_up, going_down))| {
                (going_up.clone())
                    .zip(going_down.clone())
                    .all(|(r1, r2)| r1 == r2)
            })
            .map(|(i, _)| i)?;

        debug!(actual_reflection);
        Some(Reflection::Horizontal(actual_reflection + 1))
    }

    fn find_vertical(&self) -> Option<Reflection> {
        log!("|||||||");
        let actual_reflection = self
            .grid
            .cols()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (r1, r2))| r1.clone().zip(r2.clone()).all(|(i1, i2)| i1 == i2))
            .map(|(i, _)| {
                debug!(i);
                let going_up = self.grid.cols().take(i + 1).rev();
                let mut cols = self.grid.cols();
                for _ in 0..=i {
                    cols.next();
                }
                let going_down = cols;
                (i, (going_up, going_down))
            })
            .find(|(_, (going_up, going_down))| {
                (going_up.clone())
                    .zip(going_down.clone())
                    .all(|(r1, r2)| r1.zip(r2.clone()).all(|(i1, i2)| i1 == i2))
            })
            .map(|(i, _)| i)?;

        debug!(actual_reflection);
        Some(Reflection::Vertical(actual_reflection + 1))
    }
}

enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Reflection {
    fn score(&self) -> usize {
        match self {
            Self::Vertical(x) => *x,
            Self::Horizontal(x) => 100 * x,
        }
    }
}
