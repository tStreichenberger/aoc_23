use std::{
    env,
    fs,
    io::{
        self,
        Write,
    },
    time::Instant,
};

use colored::Colorize;

mod days;
mod display;
mod logging;

use days::Day;
use logging::log;

mod prelude {
    pub use crate::{
        days::Day,
        logging::*,
    };
}

fn main() {
    logging::ChristmasLogger::init();
    set_panic_handler();
    let day_num = parse_input_day_num();

    let input = get_input(day_num);

    let day = macros::get_day!(day_num);

    println!("\n{}\n", display::santa_hat());

    run_star(day, input.clone(), false);
    run_star(day, input, true)
}

fn set_panic_handler() {
    let default_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        println!("{}", display::oops_santa());
        default_hook(panic_info)
    }))
}

fn parse_input_day_num() -> usize {
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        // prompt user if they didn't input
        print!("{}", "Enter day: ".green());
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }
    // Parse day as number
    day = day.trim().to_string();
    match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            std::process::exit(1);
        }
    }
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
