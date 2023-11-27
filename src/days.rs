pub mod day01;

pub const UNIMPLEMENTED_SOLUTION: &str = "Unimplemented";

pub trait Day {
    fn star1(&self, _input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }

    fn star2(&self, _input: String) -> String {
        UNIMPLEMENTED_SOLUTION.into()
    }
}

pub fn get_day(daynum: usize) -> Box<dyn Day> {
    match daynum {
        1 => Box::new(day01::Day01),
        4 => Box::new(Unimplemented),
        _ => panic!("No Solution found for day: {daynum}"),
    }
}

pub struct Unimplemented;
impl Day for Unimplemented {}
