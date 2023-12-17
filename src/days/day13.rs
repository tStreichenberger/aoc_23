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

    fn star2(&self, input: String) -> String {
        input
            .split("\n\n")
            .parse_each()
            .map(|m: Mirror| m.find_reflection_smudge().score())
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

    fn find_reflection_smudge(&self) -> Reflection {
        self.find_horizontal_smudge()
            .or_else(|| self.find_vertical_smudge())
            .expect("Did not find horizontal or vertical reflection")
    }

    fn find_horizontal_smudge(&self) -> Option<Reflection> {
        log!("____");
        let actual_reflection = self
            .grid
            .rows()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (r1, r2))| {
                matches!(
                    line_equal(r1.iter(), r2.iter()),
                    MirrorMatch::Match | MirrorMatch::Smudge
                )
            })
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
                let mirror_matches = going_up
                    .clone()
                    .zip(going_down.clone())
                    .map(|(r1, r2)| line_equal(r1.iter(), r2.iter()));
                let mut smudged = false;
                for mirror_match in mirror_matches {
                    match mirror_match {
                        MirrorMatch::NoMatch => return false,
                        MirrorMatch::Smudge if smudged => {
                            log!("Found Second Smudge!!!!!");
                            return false;
                        }
                        MirrorMatch::Smudge if !smudged => {
                            log!("Found First Smudge!!!!!");
                            smudged = true
                        }
                        _ => (),
                    }
                }
                smudged
            })
            .map(|(i, _)| i)?;

        debug!(actual_reflection);
        Some(Reflection::Horizontal(actual_reflection + 1))
    }

    fn find_vertical_smudge(&self) -> Option<Reflection> {
        log!("|||||||");
        let actual_reflection = self
            .grid
            .cols()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (r1, r2))| {
                matches!(
                    line_equal(r1.clone(), r2.clone()),
                    MirrorMatch::Match | MirrorMatch::Smudge
                )
            })
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
                let mirror_matches = going_up
                    .clone()
                    .zip(going_down.clone())
                    .map(|(r1, r2)| line_equal(r1, r2));
                let mut smudged = false;
                for mirror_match in mirror_matches {
                    match mirror_match {
                        MirrorMatch::NoMatch => return false,
                        MirrorMatch::Smudge if smudged => {
                            log!("Found Second Smudge!!!!!");
                            return false;
                        }
                        MirrorMatch::Smudge if !smudged => {
                            log!("Found First Smudge!!!!!");
                            smudged = true
                        }
                        _ => (),
                    }
                }
                smudged
            })
            .map(|(i, _)| i)?;

        debug!(actual_reflection);
        Some(Reflection::Vertical(actual_reflection + 1))
    }
}

fn line_equal<'a>(
    r1: impl Iterator<Item = &'a char>,
    r2: impl Iterator<Item = &'a char>,
) -> MirrorMatch {
    let num_diff = r1.zip(r2).map(|(i1, i2)| (i1 != i2) as usize).sum();
    match num_diff {
        0 => MirrorMatch::Match,
        1 => MirrorMatch::Smudge,
        _ => MirrorMatch::NoMatch,
    }
}

enum MirrorMatch {
    Smudge,
    Match,
    NoMatch,
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
