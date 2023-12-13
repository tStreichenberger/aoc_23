use std::collections::HashMap;

use crate::prelude::*;

pub struct Day08;
impl Day for Day08 {
    fn star1(&self, input: String) -> String {
        Instructions::parse(&input)
            .find_position(|node| node == &"ZZZ")
            .map(|(index, _)| index + 1)
            .unwrap()
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        let instruction = Instructions::parse(&input);
        instruction
            .nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|node| {
                instruction
                    .clone_with_starting_node(node)
                    .find_position(|node| node.ends_with('Z'))
                    .map(|(index, _)| index + 1)
                    .unwrap()
            })
            // 🙏 myscon
            .fold(1, num_integer::lcm)
            .to_string()
    }
}

struct Instructions<'a> {
    instructions: Vec<Direction>,
    nodes: HashMap<&'a str, [&'a str; 2]>,
    prev_node: &'a str,
    current_index: usize,
}

impl<'a> Instructions<'a> {
    fn parse(s: &'a str) -> Self {
        let mut lines = s.lines();
        let instructions = lines.next().unwrap().chars().map(Direction::from).collect();

        // empty line
        lines.next();

        let nodes = lines
            .map(|line| {
                let key = &line[0..3];
                let left = &line[7..10];
                let right = &line[12..15];
                (key, [left, right])
            })
            .collect();
        Self {
            instructions,
            nodes,
            prev_node: "AAA",
            current_index: 0,
        }
    }

    fn clone_with_starting_node(&self, starting_node: &'a str) -> Self {
        Self {
            instructions: self.instructions.clone(),
            nodes: self.nodes.clone(),
            prev_node: starting_node,
            current_index: self.current_index,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    L = 0,
    R = 1,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            'R' => Self::R,
            c => panic!("Could not parse direction from {c}"),
        }
    }
}

impl<'a> Iterator for Instructions<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let dir = self
            .instructions
            .get(self.current_index % self.instructions.len())
            .unwrap();
        self.current_index += 1;

        let next_node = self.nodes.get(self.prev_node).unwrap()[*dir as usize];
        self.prev_node = next_node;
        Some(next_node)
    }
}
