use std::{
    env,
    fmt::Display,
    fs,
    io,
    time::Instant,
};

use colored::Colorize;

mod days;

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }

    // Parse day as number
    day = day.trim().to_string();
    let day_num = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day_num));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    // Get corresponding function
    let to_run = days::get_day(day_num);

    // Time it

    println!("");
    println!("{}", "Running Part 1".bright_yellow());
    println!("{}", banner());
    let part1_start = Instant::now();
    let solution = to_run.star1(input.clone());
    println!("{} {solution}", "Solution:".green());
    let part1_dur = part1_start.elapsed();
    println!("{}: {part1_dur:?}", "Took".green());

    println!("");
    println!("{}", "Running Part 2".bright_yellow());
    println!("{}", banner());
    let part2_start = Instant::now();
    let solution = to_run.star2(input);
    println!("{} {solution}", "Solution:".green());
    let part2_dur = part2_start.elapsed();
    println!("{}: {part2_dur:?}", "Took".green());
    println!("")
}

fn banner() -> impl Display {
    let mut x = "-".green().to_string();
    for _ in 0..12 {
        x.push_str("-".red().to_string().as_str());
        x.push_str("-".green().to_string().as_str());
    }

    x.to_string()
}
