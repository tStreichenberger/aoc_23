use crate::prelude::*;
use std::{
    collections::{
        HashMap,
        VecDeque,
    },
    ops::Range,
};

pub struct Day19;
impl Day for Day19 {
    fn star1(&self, input: String) -> String {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows = Workflow::parse_map(workflows);
        parts
            .parsed_lines::<Part>()
            .filter(|part| check_part(&workflows, part))
            .sum_by(Part::score)
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        let (workflows, _parts) = input.split_once("\n\n").unwrap();
        let workflows = Workflow::parse_map(workflows);
        let part_range = PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };
        check_part_range(&workflows, &part_range)
            .into_iter()
            .map(|r| debug!(r))
            .sum_by(PartRange::combos)
            .to_string()
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get(&self, typ: &str) -> usize {
        match typ {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("invalid part type: {typ}"),
        }
    }

    fn score(self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn get(&self, typ: &str) -> Range<usize> {
        match typ {
            "x" => self.x.clone(),
            "m" => self.m.clone(),
            "a" => self.a.clone(),
            "s" => self.s.clone(),
            _ => panic!("invalid part type: {typ}"),
        }
    }

    fn combos(self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    #[rustfmt::skip]
    fn split_at(&self, typ: &str, num: usize) -> (PartRange, PartRange) {
        match typ {
            "x" => (Self {x: self.x.start..num, ..self.clone()}, Self {x: num..self.x.end, ..self.clone()}),
            "m" => (Self {m: self.m.start..num, ..self.clone()}, Self {m: num..self.m.end, ..self.clone()}),
            "a" => (Self {a: self.a.start..num, ..self.clone()}, Self {a: num..self.a.end, ..self.clone()}),
            "s" => (Self {s: self.s.start..num, ..self.clone()}, Self {s: num..self.s.end, ..self.clone()}),
            _ => panic!("invalid part type: {typ}"),
        }
    }
}

impl FromStr for Part {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // {x=787,m=2655,a=1222,s=2876}
        let mut nums = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|type_assign| &type_assign[2..])
            .parse_each::<usize>();

        Ok(Self {
            x: nums.next().unwrap(),
            m: nums.next().unwrap(),
            a: nums.next().unwrap(),
            s: nums.next().unwrap(),
        })
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse_map(s: &'a str) -> HashMap<&'a str, Workflow<'a>> {
        s.lines()
            .map(|line| {
                // qqz{s>2770:qs,m<1801:hdj,R}
                let (label, rules) = line.split_once('{').unwrap();
                let rules = rules
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                    .map(Rule::from)
                    .collect();
                (label, Workflow { rules })
            })
            .collect()
    }

    fn check_part(&'a self, part: &Part) -> Label<'a> {
        self.rules.iter().find_map(|rule| rule.check(part)).unwrap()
    }

    fn check_part_range(&'a self, parts: &PartRange) -> Vec<(Label<'a>, PartRange)> {
        let mut new_rules = Vec::new();
        let mut curr_parts = parts.clone();
        for rule in &self.rules {
            let CheckRange {
                passes_rule,
                fails_rule,
            } = rule.check_range(&curr_parts);
            new_rules.push((rule.label, passes_rule));
            curr_parts = fails_rule;
        }
        new_rules
    }
}

fn check_part<'a>(map: &HashMap<&'a str, Workflow<'a>>, part: &Part) -> bool {
    let mut workflow = map.get("in").unwrap();
    loop {
        let next_label = workflow.check_part(part);
        match next_label {
            Label::Accept => return true,
            Label::Reject => return false,
            Label::Label(next) => workflow = map.get(next).unwrap(),
        }
    }
}

fn check_part_range<'a>(map: &HashMap<&'a str, Workflow<'a>>, parts: &PartRange) -> Vec<PartRange> {
    let mut pasing_parts = Vec::new();
    let mut parts_to_check = VecDeque::new();
    parts_to_check.push_back(("in", parts.clone()));
    while let Some((label, curr_range)) = parts_to_check.pop_front() {
        let next_parts = map.get(label).unwrap().check_part_range(&curr_range);
        for (label, range) in next_parts {
            match label {
                Label::Accept => pasing_parts.push(range),
                Label::Reject => continue,
                Label::Label(next) => parts_to_check.push_back((next, range)),
            }
        }
    }
    pasing_parts
}

struct Rule<'a> {
    cmp: fn(&usize, &usize) -> bool,
    part_type: &'a str,
    other: usize,
    label: Label<'a>,
}

impl<'a> Rule<'a> {
    fn terminal(label: &'a str) -> Self {
        Self {
            cmp: terminal_tester,
            part_type: "x",
            other: 1,
            label: label.into(),
        }
    }

    fn check(&self, part: &Part) -> Option<Label> {
        (self.cmp)(&part.get(self.part_type), &self.other).then_some(self.label)
    }

    fn check_range(&self, part: &PartRange) -> CheckRange {
        #[allow(clippy::fn_address_comparisons)]
        // clippy is probably right to deny this but lets try it anyway
        let offset = (self.cmp == usize::gt) as usize;
        let (lt, gt) = part.split_at(self.part_type, self.other + offset);
        match (self.cmp)(&gt.get(self.part_type).start, &self.other) {
            true => CheckRange {
                passes_rule: gt,
                fails_rule: lt,
            },
            false => CheckRange {
                passes_rule: lt,
                fails_rule: gt,
            },
        }
    }
}

struct CheckRange {
    passes_rule: PartRange,
    fails_rule: PartRange,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let Some((compare, label)) = s.split_once(':') else {
            return Self::terminal(s);
        };
        let part_type = &compare[0..1];
        let cmp = match &compare[1..2] {
            ">" => usize::gt,
            "<" => usize::lt,
            x => panic!("Invalid comarison operator: {x}"),
        };
        let other = compare[2..].parse().unwrap();
        Self {
            part_type,
            cmp,
            label: label.into(),
            other,
        }
    }
}

#[derive(Clone, Copy)]
enum Label<'a> {
    Accept,
    Reject,
    Label(&'a str),
}

impl<'a> From<&'a str> for Label<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Label(s),
        }
    }
}

fn terminal_tester(_: &usize, _: &usize) -> bool {
    true
}
