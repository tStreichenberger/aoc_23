use crate::prelude::*;

pub struct Day09;
impl Day for Day09 {
    fn star1(&self, input: String) -> String {
        input
            .parsed_lines()
            .map(|h: History| find_next(h.values.into_iter()))
            .sum::<isize>()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        input
            .parsed_lines()
            .map(|h: History| find_prev(h.values))
            .sum::<isize>()
            .to_string()
    }
}

struct History {
    values: Vec<isize>,
}

impl FromStr for History {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.split_ascii_whitespace().parse_each().collect(),
        })
    }
}

fn find_next(iter: impl Iterator<Item = isize> + Clone) -> isize {
    if iter.clone().all(|i| i == 0) {
        return 0;
    }
    let last = iter.clone().last().unwrap();
    // the whole point of making this function take in an iter was to avoid doing this collect.
    // But if I just keep passing the iters I hit the recursion limit :sadge:
    let diffs = iter.tuple_windows().map(|(i1, i2)| i2 - i1).collect_vec();
    last + find_next(diffs.into_iter())
}

fn find_prev(vals: Vec<isize>) -> isize {
    if vals.iter().all(|i| *i == 0) {
        return 0;
    }
    let first = vals[0];
    let diffs = vals
        .into_iter()
        .tuple_windows()
        .map(|(i1, i2)| i2 - i1)
        .collect_vec();
    first - find_prev(diffs)
}
