use crate::prelude::*;
use index_list::{
    IndexList,
    ListIndex,
};
use std::num::Wrapping;

pub struct Day15;
impl Day for Day15 {
    fn star1(&self, input: String) -> String {
        input.split(',').map(hash).sum::<usize>().to_string()
    }

    fn star2(&self, input: String) -> String {
        let mut map = HashMap::new();
        input
            .split(',')
            .parse_each()
            .for_each(|inst| map.do_instruction(inst));
        map.focusing_power().to_string()
    }
}

/// - Determine the ASCII code for the current character of the string.
/// - Increase the current value by the ASCII code you just determined.
/// - Set the current value to itself multiplied by 17.
fn hash(s: &str) -> usize {
    let mut h = Wrapping(0_u8);
    for c in s.chars() {
        if c == '\n' {
            continue;
        }
        h += c as u8;
        h *= 17;
    }
    h.0 as usize
}

enum Instruction {
    Insert(LabeledLens),
    Remove(String),
}

impl FromStr for Instruction {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(['=', '-']);
        let label = split.next().unwrap();
        match split.next().and_then(|maybe_num| maybe_num.parse().ok()) {
            None => Ok(Self::Remove(label.into())),
            Some(focal_length) => Ok(Self::Insert(LabeledLens {
                label: label.into(),
                length: focal_length,
            })),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct LabeledLens {
    label: String,
    length: usize,
}

struct HashMap {
    buckets: Vec<IndexList<LabeledLens>>,
}

impl HashMap {
    fn new() -> Self {
        Self {
            buckets: (0..256).map(|_| IndexList::new()).collect(),
        }
    }

    fn do_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Remove(label) => self.remove(&label),
            Instruction::Insert(lens) => self.insert(lens),
        }
    }

    fn insert(&mut self, lens: LabeledLens) {
        let Some(i) = self.get_lens(&lens.label) else {
            self.mut_bucket(&lens.label).insert_last(lens);
            return;
        };

        let bucket = self.mut_bucket(&lens.label);
        bucket.insert_after(i, lens);
        bucket.remove(i);
    }

    fn remove(&mut self, label: &str) {
        let Some(i) = self.get_lens(label) else {
            return;
        };
        let bucket = self.mut_bucket(label);

        bucket.remove(i);
    }

    fn get_lens(&self, label: &str) -> Option<ListIndex> {
        let hash = hash(label);
        let bucket = self.buckets.get(hash).unwrap();
        let elem = bucket
            .into_iter()
            .find(|labeled_lens| labeled_lens.label == label)?;
        let index = bucket.index_of(elem.clone());
        Some(index)
    }

    fn mut_bucket(&mut self, label: &str) -> &mut IndexList<LabeledLens> {
        let hash = hash(label);
        self.buckets.get_mut(hash).unwrap()
    }

    fn focusing_power(&self) -> usize {
        self.buckets.iter().enumerate().sum_by(|(box_num, bucket)| {
            bucket
                .iter()
                .enumerate()
                .sum_by(|(lens_num, lens)| (box_num + 1) * (lens_num + 1) * lens.length)
        })
    }
}
