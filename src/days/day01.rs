use crate::prelude::*;

pub struct Day01;

impl Day for Day01 {
    fn star1(&self, _input: String) -> String {
        let x = 42;
        let y = debug!(x + 3);
        debug!(y);
        log!("added y successfully");
        return format!("{y}");
    }
}
