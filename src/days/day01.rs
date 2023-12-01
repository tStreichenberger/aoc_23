use regex::Regex;

use crate::prelude::*;

pub struct Day01;

impl Day for Day01 {
    fn star1(&self, input: String) -> String {
        let sum = input.lines().fold(0, |sum, line| {
            sum + line.parse::<CalibrationValue>().unwrap().0
        });
        return sum.to_string();
    }

    fn star2(&self, input: String) -> String {
        let sum = input.lines().fold(0, |sum, line| {
            sum + line.parse::<FancyCalibrationValue>().unwrap().0
        });
        return sum.to_string();
    }
}

struct CalibrationValue(u32);

impl FromStr for CalibrationValue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = true;
        let mut val = 0;
        let mut last = 0;
        for char in s.chars() {
            if let Some(num) = char.to_digit(10) {
                last = num;
                if first {
                    first = false;
                    val = num * 10;
                }
            }
        }
        val += last;
        Ok(Self(val))
    }
}

lazy_static! {
    static ref NUMBERS_REGEX: Regex =
        Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    static ref SREBMUN_REGEX: Regex =
        Regex::new("enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|[0-9]").unwrap();
}

struct FancyCalibrationValue(u32);

impl FromStr for FancyCalibrationValue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = parse_maybe_written_number(NUMBERS_REGEX.find(s).unwrap().as_str());
        let last = SREBMUN_REGEX
            .find(&s.chars().rev().collect::<String>())
            .map(|m| parse_maybe_written_number(&m.as_str().chars().rev().collect::<String>()))
            .unwrap_or(first);
        Ok(Self(first * 10 + last))
    }
}

fn parse_maybe_written_number(maybe_written: &str) -> u32 {
    maybe_written
        .parse()
        .unwrap_or_else(|_| parse_written_number(maybe_written))
}

fn parse_written_number(written: &str) -> u32 {
    match written {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Recieved invalid written digit {written}"),
    }
}
