use std::{env, time::Instant};

mod days;
use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day> [test]");
        return;
    }

    let day = &args[1].parse::<u8>().unwrap_or(0);
    let is_test = args.get(2).map(|s| s == "test").unwrap_or(false);
    let file_path = if is_test {
        format!("year2024/input_test/{:02}.txt", day)
    } else {
        format!("year2024/input/{:02}.txt", day)
    };

    let now = Instant::now();

    match day {
        1 => day01::solve(&file_path),
        2 => day02::solve(&file_path),
        3 => day03::solve(&file_path),
        4 => day04::solve(&file_path),
        5 => day05::solve(&file_path),
        6 => day06::solve(&file_path),
        7 => day07::solve(&file_path),
        8 => day08::solve(&file_path),
        9 => day09::solve(&file_path),
        10..=25 => println!("Day {day} not implemented yet."),
        _ => eprintln!("Invalid day: {}", day),
    }

    let elapsed = now.elapsed();
    println!("Day {}{} ran in {:?}", day, if is_test {" test"} else {""}, elapsed)
}
