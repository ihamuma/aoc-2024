use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 && args[1] == "test" {
        "day_2/test_input.txt"
    } else {
        "day_2/input.txt"
    };

    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect();

    let reports: Vec<Vec<i8>> = by_line
        .iter()
        .map(|line| {
            line.split(" ")
                .into_iter()
                .map(|s| s.trim().parse::<i8>().unwrap())
                .collect()
        })
        .collect();

    let mut safes: u16 = 0;
    let mut unsafes: Vec<Vec<i8>> = Vec::new();

    for report in reports {
        if check_safety(&report) {
            safes += 1
        } else {
            unsafes.push(report);
        }
    }

    println!("There are {} safe reports", safes);
    println!("{} reports are currently unsafe", unsafes.len());

    let mut dampened_safes: u16 = 0;
    for report in unsafes {
        if check_dampened_safety(&report) {
            dampened_safes += 1
        }
    }

    println!(
        "Of the unsafe reports, {} are dampened safe",
        dampened_safes
    );
    println!(
        "In total, there are {} safe reports",
        safes + dampened_safes
    )
}

fn check_safety(report: &Vec<i8>) -> bool {
    let ascending: bool = report[0] < report[1];

    if ascending {
        for i in 1..report.len() {
            if report[i - 1] >= report[i] || 3 < (report[i - 1] - report[i]).abs() {
                return false;
            }
        }
    } else {
        for i in 1..report.len() {
            if report[i - 1] <= report[i] || 3 < (report[i - 1] - report[i]).abs() {
                return false;
            }
        }
    }

    true
}

fn check_dampened_safety(report: &Vec<i8>) -> bool {
    for i in 0..report.len() {
        let mut test = report.clone();
        test.remove(i);
        if check_safety(&test) {
            return true;
        }
    }

    false
}
