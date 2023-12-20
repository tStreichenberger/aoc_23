use crate::prelude::*;
use std::collections::VecDeque;

pub struct Day18;
impl Day for Day18 {
    fn star1(&self, input: String) -> String {
        Trenches::new(input.parsed_lines()).area().to_string()
    }

    fn star2(&self, input: String) -> String {
        let insts = input
            .parsed_lines()
            .map(|inst: DigInstruction| inst.into_color_mode());
        Trenches::new(insts).area().to_string()
    }
}

fn cross_product(i1: SIndex, i2: SIndex) -> i64 {
    i1.0 * i2.1 - i1.1 * i2.0
}

fn distance(i1: SIndex, i2: SIndex) -> i64 {
    // this is generally wrong but is correct if we assume that
    // these two coords are vertical or horizontal of each other
    ((i1.0 - i2.0) + (i1.1 - i2.1)).abs()
}

#[derive(Clone)]
struct Trenches {
    insts: VecDeque<DigInstruction>,
    current_pos: SIndex,
}

impl Iterator for Trenches {
    type Item = SIndex;
    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.insts.pop_front().map(|inst| {
            std::iter::repeat(inst.dir)
                .take(inst.amount)
                .fold(self.current_pos, |loc, dir| dir.sgo(loc))
        })?;

        self.current_pos = next_pos;
        Some(next_pos)
    }
}

impl Trenches {
    fn new(insts: impl Iterator<Item = DigInstruction>) -> Self {
        Self {
            insts: insts.collect(),
            current_pos: (0, 0),
        }
    }

    fn area(self) -> i64 {
        let mut iter = self.peekable();
        let first = *iter.peek().unwrap();
        let (area_2, perim) = iter
            .chain(std::iter::once(first))
            .tuple_windows()
            .map(|(i1, i2)| (cross_product(i1, i2), distance(i1, i2)))
            .fold((0, 0), |(c_sum, p_sum), (c, p)| (c_sum + c, p_sum + p));
        area_2.abs() / 2 + perim / 2 + 1
    }
}

#[derive(Debug, Clone)]
struct DigInstruction {
    dir: Direction,
    amount: usize,
    color_dist: usize,
    color_dir: Direction,
}

impl DigInstruction {
    fn into_color_mode(self) -> Self {
        Self {
            dir: self.color_dir,
            amount: self.color_dist,
            ..self
        }
    }
}

impl FromStr for DigInstruction {
    type Err = Infallible;
    /// L 4 (#327cb0)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let dir = split.next().unwrap().chars().next().unwrap().into();
        let amount = split.next().unwrap().parse().unwrap();
        let color = &split.next().unwrap()[2..8];
        let color_dist = usize::from_str_radix(&color[0..5], 16).unwrap();
        let color_dir = match &color[5..6] {
            "0" => Direction::West,
            "1" => Direction::South,
            "2" => Direction::East,
            "3" => Direction::North,
            x => panic!("invalid color dist: {x}"),
        };
        Ok(Self {
            dir,
            amount,
            color_dir,
            color_dist,
        })
    }
}
