use std::collections::HashMap;

use crate::prelude::*;

pub struct Day08;
impl Day for Day08 {
    fn star1(&self, input: String) -> String {
        Instructions::new()
            .find_position(|node| node == &"ZZZ")
            .map(|(index, _)| index)
            .unwrap()
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
    fn new() -> Self {
        todo!()
    }
}

#[derive(Clone, Copy)]
enum Direction {
    L = 0,
    R = 1,
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
