use crate::prelude::*;
use std::collections::{
    BinaryHeap,
    HashSet,
};

pub struct Day17;
impl Day for Day17 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<City>()
            .unwrap()
            .find_path(City::get_next_dist)
            .to_string()
    }
    fn star2(&self, input: String) -> String {
        input
            .parse::<City>()
            .unwrap()
            .find_path(City::get_next_dist_ultra)
            .to_string()
    }
}

struct City {
    streets: Grid<u8>,
}

/// Node for the priority queue
#[derive(Eq, Clone, Debug)]
struct Node {
    heat_loss: usize,
    longest_straight_path: u8,
    arrived_facing: Direction,
    pos: Index,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss.eq(&other.heat_loss)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

/// Node for the visited hashset. Has diverent equality impl
#[derive(PartialEq, Eq, Hash)]
struct VisitedNode {
    pos: Index,
    arrived_facing: Direction,
    longest_straight_path: u8,
}

impl From<Node> for VisitedNode {
    fn from(n: Node) -> Self {
        Self {
            pos: n.pos,
            longest_straight_path: n.longest_straight_path,
            arrived_facing: n.arrived_facing,
        }
    }
}

impl City {
    fn find_path(&self, next_pass_fn: impl Fn(Direction, &Node) -> Option<u8>) -> usize {
        let mut current_node = Node {
            heat_loss: 0,
            longest_straight_path: 0,
            // this could be South or East
            arrived_facing: Direction::South,
            pos: (0, 0),
        };
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(current_node.clone());

        let final_pos = (self.streets.num_rows() - 1, self.streets.num_cols() - 1);

        let mut visited_nodes = HashSet::new();

        while let Some(node) = priority_queue.pop() {
            if node.pos == final_pos {
                current_node = node;
                break;
            }

            let visited_node: VisitedNode = node.clone().into();
            if !visited_nodes.insert(visited_node) {
                continue;
            }

            for dir in self.next_dirs(node.pos, node.arrived_facing) {
                let Some(next_dist) = next_pass_fn(dir, &node) else {
                    continue;
                };

                let next_pos = dir.go(node.pos);
                let heat_loss = *self.streets.get(next_pos).unwrap() as usize + node.heat_loss;
                priority_queue.push(Node {
                    heat_loss,
                    longest_straight_path: next_dist,
                    arrived_facing: dir,
                    pos: next_pos,
                })
            }
        }

        current_node.heat_loss
    }

    /// Returns None if its not a valid dir
    fn get_next_dist(going_to: Direction, current_node: &Node) -> Option<u8> {
        match (
            going_to == current_node.arrived_facing,
            current_node.longest_straight_path,
        ) {
            (true, 0..=2) => Some(current_node.longest_straight_path + 1),
            (false, _) => Some(1),
            _ => None,
        }
    }

    /// Returns None if its not a valid dir
    fn get_next_dist_ultra(going_to: Direction, current_node: &Node) -> Option<u8> {
        match (
            going_to == current_node.arrived_facing,
            current_node.longest_straight_path,
        ) {
            (true, 0..=9) => Some(current_node.longest_straight_path + 1),
            (false, 4..) => Some(1),
            // edge case for the first node
            (false, 0) => Some(1),
            _ => None,
        }
    }

    fn next_dirs(&self, pos: Index, arrived_facing: Direction) -> Vec<Direction> {
        [
            arrived_facing,
            arrived_facing.right_dir(),
            arrived_facing.left_dir(),
        ]
        .into_iter()
        .filter(|dir| self.streets.get(dir.go(pos)).is_some())
        .collect()
    }
}

struct Street(u8);

impl From<char> for Street {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap() as u8)
    }
}

impl FromStr for City {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            streets: s
                .parse::<Grid<Street>>()?
                .into_iter()
                .map(|row| row.into_iter().map(|v| v.0).collect())
                .collect(),
        })
    }
}
