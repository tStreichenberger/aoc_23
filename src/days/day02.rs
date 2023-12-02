use crate::prelude::*;

pub struct Day02;
impl Day for Day02 {
    fn star1(&self, input: String) -> String {
        input
            .lines()
            .map(Game::from_str)
            .filter_ok(|game| {
                game.subsets.iter().all(|subset| {
                    subset.red <= RED_LIMIT
                        && subset.green <= GREEN_LIMIT
                        && subset.blue <= BLUE_LIMIT
                })
            })
            .fold_ok(0, |sum, game| sum + game.id)
            .unwrap()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        input
            .lines()
            .map(Game::from_str)
            .fold_ok(0, |sum, game| sum + game.power())
            .unwrap()
            .to_string()
    }
}

const RED_LIMIT: usize = 12;
const GREEN_LIMIT: usize = 13;
const BLUE_LIMIT: usize = 14;

struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

impl FromStr for Game {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_header, subsets) = s.split_once(": ").unwrap();

        let id = game_header[5..].parse().unwrap();

        let subsets = subsets
            .split("; ")
            .map(Subset::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        Ok(Self { id, subsets })
    }
}

impl Game {
    fn min_cubes(&self) -> Subset {
        self.subsets
            .iter()
            .fold(Subset::empty(), |mut min_cubes, set| {
                min_cubes.red = min_cubes.red.max(set.red);
                min_cubes.green = min_cubes.green.max(set.green);
                min_cubes.blue = min_cubes.blue.max(set.blue);
                min_cubes
            })
    }

    fn power(&self) -> usize {
        let min_cubes = self.min_cubes();
        min_cubes.red * min_cubes.green * min_cubes.blue
    }
}

struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

impl Subset {
    fn empty() -> Self {
        Subset {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl FromStr for Subset {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut subset = Subset::empty();
        for handful in s.split(", ") {
            let (num, color) = handful.split_once(" ").unwrap();
            let num = num.parse().unwrap();
            match color {
                "red" => subset.red = num,
                "green" => subset.green = num,
                "blue" => subset.blue = num,
                _ => panic!("Tried to parse color but received: {color}"),
            }
        }
        Ok(subset)
    }
}
