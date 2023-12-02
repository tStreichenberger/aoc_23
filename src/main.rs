use std::{
    env,
    fs,
    time::Instant,
};

use clap::Parser;
use colored::Colorize;

mod days;
mod display;
mod logging;
mod ext;

use days::Day;
use logging::log;

mod prelude {
    pub use crate::{
        days::Day,
        logging::*,
        ext::*,
    };
    pub use itertools::Itertools;
    pub use lazy_static::lazy_static;
    pub use std::{
        convert::Infallible,
        str::FromStr,
    };
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    stress_test: bool,
    #[arg(short = 'l', long, default_value_t = 1000)]
    test_len: u32,
    day_num: usize,
}

lazy_static::lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
    logging::ChristmasLogger::init();
    set_panic_handler();
    let day_num = ARGS.day_num;

    let input = get_input(day_num);

    let day = macros::get_day!(day_num);

    println!("\n{}\n", display::santa_hat());

    let to_run = match ARGS.stress_test {
        false => run_star,
        true => {
            log!("Stress Testing over {} runs", ARGS.test_len);
            stress_test_star
        }
    };

    to_run(day, input.clone(), false);
    to_run(day, input, true)
}

fn set_panic_handler() {
    let default_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        println!("{}", display::oops_santa());
        default_hook(panic_info)
    }))
}

fn get_input(day_num: usize) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day_num));
    log!("Reading {}\n", filename.display());
    fs::read_to_string(filename).expect("Error while reading")
}

fn run_star(day: &dyn Day, input: String, is_second_star: bool) {
    let day_num = is_second_star as usize + 1;

    println!(
        "{} {} {}",
        "Running".green(),
        "*".bright_yellow(),
        day_num.to_string().green()
    );
    println!("{}", display::banner());
    let start = Instant::now();
    let solution = match is_second_star {
        false => day.star1(input),
        true => day.star2(input),
    };
    println!("{} {solution}", "Solution:".green());
    let dur = start.elapsed();
    println!("{}: {dur:?}", "Took".green());
    println!("");
}

fn stress_test_star(day: &dyn Day, input: String, is_second_star: bool) {
    let day_num = is_second_star as usize + 1;
    let mut total = std::time::Duration::ZERO;
    for _ in 0..ARGS.test_len {
        total += time_star(day, input.clone(), is_second_star);
    }
    log!("Ran star {} in {:?}", day_num, total / ARGS.test_len);
}

fn time_star(day: &dyn Day, input: String, is_second_star: bool) -> std::time::Duration {
    let start = std::time::Instant::now();
    match is_second_star {
        false => day.star1(input),
        true => day.star2(input),
    };
    start.elapsed()
}
