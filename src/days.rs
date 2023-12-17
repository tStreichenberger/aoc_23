pub mod day14;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

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
