use crate::prelude::*;
use itertools::EitherOrBoth;

pub struct Day12;
impl Day for Day12 {
    fn star1(&self, input: String) -> String {
        input
            .parsed_lines()
            .map(|engine: Engine| {
                permutations(engine.parts.clone().into_iter())
                    .iter()
                    .filter(|parts| engine.is_valid(parts))
                    .count()
            })
            .sum::<usize>()
            .to_string()
        // let engine = "?###???????? 3,2,1".parse::<Engine>().unwrap();
        // print_parts(&engine.parts);
        // println!();
        // let perms = permutations(engine.parts.clone().into_iter());
        // for perm in &perms {
        //     print_parts(perm);
        // }
        // println!("\n\n");
        // perms.iter().filter(|perm| engine.is_valid(perm)).for_each(|perm| print_parts(perm));
        // "HI".into()
    }

    fn star2(&self, input: String) -> String {
        input
            .parsed_lines()
            .map(|engine: Engine| engine.expand())
            .map(|engine| {
                permutations(engine.parts.clone().into_iter())
                    .iter()
                    .filter(|parts| engine.is_valid(parts))
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }
}

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
    fn is_valid(&self, parts: &[Part]) -> bool {
        parts
            .iter()
            .group_by(|part| *part)
            .into_iter()
            .filter(|(part, _group)| matches!(part, Part::Broken))
            .map(|(_, group)| group.count())
            .zip_longest(&self.broken_parts)
            .all(|both| matches!(both, EitherOrBoth::Both(x, y) if x == *y))
    }

    fn expand(self) -> Self {
        let parts = std::iter::repeat(self.parts)
            .take(5)
            .interleave(std::iter::repeat(vec![Part::Mystery]).take(4))
            .flatten()
            .collect();
        let broken_parts = std::iter::repeat(self.broken_parts)
            .take(5)
            .flatten()
            .collect();
        Self {
            parts,
            broken_parts,
        }
    }
}

fn permutations(mut parts: impl Iterator<Item = Part>) -> Vec<Vec<Part>> {
    let Some(part) = parts.next() else {
        return Vec::new();
    };
    let next_part_options = permutations(parts);
    let this_part = if let Part::Mystery = part {
        vec![Part::Broken, Part::Fixed]
    } else {
        vec![part]
    };
    if next_part_options.is_empty() {
        return this_part.into_iter().map(|part| vec![part]).collect();
    }
    this_part
        .into_iter()
        .flat_map(|part| {
            next_part_options
                .iter()
                .map(move |parts| std::iter::once(part).chain(parts.iter().copied()).collect())
        })
        .collect()
}

fn print_parts(parts: &[Part]) {
    for part in parts {
        print!(
            "{}",
            match part {
                Part::Broken => '#',
                Part::Mystery => '?',
                Part::Fixed => '.',
            }
        )
    }
    println!()
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
