use crate::prelude::*;

pub struct Day06;
impl Day for Day06 {
    fn star1(&self, input: String) -> String {
        parse_races(&input)
            .into_iter()
            .map(|race| race.get_num_winning_times())
            .fold(1, |product, num_wins| product * num_wins)
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        input
            .parse::<Race>()
            .unwrap()
            .get_num_winning_times()
            .to_string()
    }
}

struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn get_num_winning_times(&self) -> usize {
        let (lower_bound, upper_bound) = get_times(self.time, self.record);
        let lower = lower_bound.ceil() as usize;
        let upper = upper_bound.floor() as usize;
        upper - lower + 1
    }
}

impl FromStr for Race {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .join("")
            .parse()
            .unwrap();
        let record = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .join("")
            .parse()
            .unwrap();
        Ok(Self { time, record })
    }
}

fn parse_races(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .parse_each();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .parse_each();
    times
        .zip(distances)
        .map(|(time, record)| Race { time, record })
        .collect()
}

/// Takes in race time and record. Returns the 2 possible times that could have set that record
fn get_times(time: usize, record: usize) -> (f64, f64) {
    // and who said we would never use quadtratic formula in the real world
    let time = time as f64;
    let record = record as f64;
    let half_time = time / 2.;
    let quadrat = f64::sqrt(time.powi(2) - 4. * record) / 2.;
    (half_time - quadrat, half_time + quadrat)
}
