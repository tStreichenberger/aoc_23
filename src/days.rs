pub mod day01;
pub mod day02;

pub const UNIMPLEMENTED_SOLUTION: &str = "Unimplemented";

pub trait Day {
    fn star1(&self, _input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }

    fn star2(&self, _input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }
}
