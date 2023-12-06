use crate::prelude::*;
use std::{
    collections::HashMap,
    ops::Range,
};

use rayon::prelude::{
    ParallelBridge,
    ParallelIterator,
};

pub struct Day05;
impl Day for Day05 {
    fn star1(&self, input: String) -> String {
        let almanac = Almanac::parse(&input);
        almanac
            .starting_seeds
            .iter()
            .map(|seed| almanac.get_seed_location(*seed))
            .min()
            .unwrap()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        let almanac = Almanac::parse(&input);
        almanac
            .starting_seeds
            .iter()
            .tuples::<(_, _)>()
            // sometimes the right answer is to just throw more compute at the problem
            .par_bridge()
            .flat_map(|(start, len)| (*start..*start + len))
            .map(|seed| almanac.get_seed_location(seed))
            .min()
            .unwrap()
            .to_string()
    }
}

struct Almanac<'a> {
    starting_seeds: Vec<usize>,
    maps: HashMap<&'a str, Map<'a>>,
}

impl<'a> Almanac<'a> {
    fn get_seed_location(&self, seed: usize) -> usize {
        self.get_location_for("seed", seed)
    }
    fn get_location_for(&self, typ: &str, item: usize) -> usize {
        let map = self.maps.get(typ).unwrap();
        let converted = map.convert(item);
        match map.to {
            "location" => converted,
            to => self.get_location_for(to, converted),
        }
    }

    /*
    For the smarter solution that I could not get to work
    fn get_range_permutations(&self, typ: &str, range: Range<usize>) -> Vec<Range<usize>> {
        let map = self.maps.get(typ).unwrap();
        let ranges = map.ranges.iter().map(|conversion| &conversion.range);
        let perms = get_range_subsections(range.clone(), ranges);
        match map.to {
            "location" => perms,
            to => perms
                .into_iter()
                .map(|range| map.convert_range(range))
                .flat_map(|range| self.get_range_permutations(to, range))
                .map(|range| map.devert_range(range))
                .collect(),
        }
    }

    // takes in seed ranges.
    // Breaks it into alternating sections that overlap
    // and don't overlap with the seed map ranges of the almanac
    fn get_range_subsections(&self, range: Range<usize>) -> impl IntoIterator<Item = Range<usize>> {
        self.get_range_permutations("seed", range)
    }
    */

    fn parse(s: &'a str) -> Self {
        let mut sections = s.split("\n\n");
        let starting_seeds = sections
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .parse_each()
            .collect();
        let maps = sections.map(Map::parse).collect();
        Self {
            maps,
            starting_seeds,
        }
    }
}
/*fn get_range_subsections<'a>(
    range: Range<usize>,
    sub_ranges: impl Iterator<Item = &'a Range<usize>>,
) -> Vec<Range<usize>> {
    let mut ranges = vec![range];
    for sub_range in sub_ranges {
        let mut new_ranges = Vec::new();
        for range in ranges {
            match overlaps(&range, &sub_range) {
                Overlap::Without => new_ranges.push(range),
                Overlap::Within => {
                    new_ranges.push(range.start..sub_range.start);
                    new_ranges.push(sub_range.clone());
                    new_ranges.push(sub_range.end..range.end);
                }
                Overlap::Left => {
                    let (first_half, second_half) = divide_range(range, sub_range.end);
                    new_ranges.push(first_half);
                    new_ranges.push(second_half);
                }
                Overlap::Right => {
                    let (first_half, second_half) = divide_range(range, sub_range.start);
                    new_ranges.push(first_half);
                    new_ranges.push(second_half);
                }
            }
        }
        ranges = new_ranges
    }

    ranges
}

/// Within:
/// [   r1   ]
///    [ r2 ]
/// No:
/// [ r1 ]
///          [ r2 ]
/// Without:
///     [ r1 ]
///   [   r2    ]
/// Left:
///    [   r1   ]
///  [  r2  ]
/// Right:
///  [    r1    ]
///      [    r2    ]
fn overlaps(r1: &Range<usize>, r2: &Range<usize>) -> Overlap {
    use Ordering::*;
    match (r1.start.cmp(&r2.start), r1.end.cmp(&r2.end)) {
        (Less, Greater) => Overlap::Within,
        (Less, Less) if r1.end > r2.start => Overlap::Right,
        (Greater, Greater) if r1.start < r2.end => Overlap::Left,
        _ => Overlap::Without,
    }
}

fn divide_range(range: Range<usize>, num: usize) -> (Range<usize>, Range<usize>) {
    (range.start..num, num..range.end)
}

#[derive(Debug)]
enum Overlap {
    Within,
    Without,
    Left,
    Right,
}
*/

struct Map<'a> {
    to: &'a str,
    ranges: Vec<Conversion>,
}
impl<'a> Map<'a> {
    fn convert(&self, num: usize) -> usize {
        self.ranges
            .iter()
            .find(|range| range.contains(&num))
            .map(|range| range.convert(num))
            .unwrap_or(num)
    }

    /// this only works if you have already made this a continuous range. See [`get_range_subsections`]
    // fn convert_range(&self, range: Range<usize>) -> Range<usize> {
    //     self.convert(range.start)..(self.convert(range.end-1)+1)
    // }

    // fn devert(&self, num: usize) -> usize {
    //     self.ranges.iter()
    //         .find(|range| range.contains_dest(&num))
    //         .map(|range| range.devert(num))
    //         .unwrap_or(num)
    // }

    // fn devert_range(&self, range: Range<usize>) -> Range<usize> {
    //     self.devert(range.start)..self.devert(range.end-1) + 1
    // }

    /// FromStr does not allow us to use lifetime of str
    /// Also want to return the from type here
    fn parse(s: &'a str) -> (&'a str, Self) {
        let mut lines_iter = s.lines();
        let (from, to) = lines_iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .split_once("-to-")
            .unwrap();
        let map = Self {
            to,
            ranges: lines_iter.parse_each().collect(),
        };
        (from, map)
    }
}

struct Conversion {
    range: Range<usize>,
    dest_start: usize,
}

impl Conversion {
    fn contains(&self, num: &usize) -> bool {
        self.range.contains(num)
    }

    // fn contains_dest(&self, num: &usize) -> bool {
    //     let len = self.range.end - self.range.start;
    //     (self.dest_start..self.dest_start+len).contains(num)
    // }

    fn convert(&self, num: usize) -> usize {
        num + self.dest_start - self.range.start
    }

    //     fn convert_range(&self, range: Range<usize>) -> Range<usize> {
    //         self.convert(range.start)..self.convert(range.end)
    //     }

    //     fn devert(&self, num: usize) -> usize {
    //         num + self.range.start - self.dest_start
    //     }

    //     fn devert_range(&self, range: Range<usize>) -> Range<usize> {
    //         self.devert(range.start)..self.devert(range.end)
    //     }
}

impl FromStr for Conversion {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let dest_start: usize = split.next().unwrap().parse().unwrap();
        let src_start: usize = split.next().unwrap().parse().unwrap();
        let range_len: usize = split.next().unwrap().parse().unwrap();
        let range = src_start..src_start + range_len;
        Ok(Self { range, dest_start })
    }
}
