use crate::prelude::*;
use itertools::EitherOrBoth;
pub struct Day12;
impl Day for Day12 {}

/*
?#??##?..???.?? 6,1,1
?#?#?#????????. 8,1
*/

struct Engine {
    parts: Vec<Part>,
    broken_parts: Vec<usize>,
}

impl FromStr for Engine {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (parts_str, broken_parts_str) = s.split_once(' ').unwrap();
        let parts = parts_str.chars().map(Part::from).collect();
        let broken_parts = broken_parts_str.split(',').parse_each().collect();
        Ok(Self {
            parts,
            broken_parts,
        })
    }
}

impl Engine {
    fn is_valid(&self, parts: &Vec<Part>) -> bool {
        parts
            .into_iter()
            .group_by(|part| *part)
            .into_iter()
            .filter(|(part, group)| matches!(part, Part::Broken))
            .map(|(_, group)| group.count())
            .zip_longest(&self.broken_parts)
            .all(|both| matches!(both, EitherOrBoth::Both(x, y) if x == *y))
    }
}

fn permutations(parts: impl Iterator<Item = Part> + Clone) -> impl Iterator<Item = impl Iterator<Item = Part>> {
    // for part in parts {
    //     if let Part::Mystery = part {
    //         todo!()
    //     } else {
    //         todo!()
    //     }
    // }
    // todo!()
    std::iter::once(std::iter::once(Part::Broken))
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Part {
    Broken,
    Fixed,
    Mystery,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '?' => Self::Mystery,
            '.' => Self::Fixed,
            c => panic!("Failed to parse from part from char: {c}"),
        }
    }
}
