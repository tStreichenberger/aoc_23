pub mod day01;
pub mod day02;

pub const UNIMPLEMENTED_SOLUTION: &str = "Unimplemented";

pub trait Day {
    #[allow(unused)]
    fn star1(&self, input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }

    #[allow(unused)]
    fn star2(&self, input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }
}
