use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";

    let by_line: Vec<String> = file_to_string_vec(path);

    let reports: Vec<Vec<i64>> = split_to_int_vecs(by_line);

    let mut safes: u32 = 0;
    let mut unsafes: Vec<Vec<i64>> = Vec::new();

    for report in reports {
        if check_safety(&report) {
            safes += 1
        } else {
            unsafes.push(report);
        }
    }

    println!("There are {} safe reports", safes);
    println!("{} reports are currently unsafe", unsafes.len());

    let mut dampened_safes: u32 = 0;
    for report in unsafes {
        if check_dampened_safety(&report) {
            dampened_safes += 1
        }
    }

    println!("Of the unsafe reports, {} are dampened safe", dampened_safes);
    println!("In total, there are {} safe reports", safes + dampened_safes)
    
}

fn file_to_string_vec(path: &str) -> Vec<String> {
    let file: File = File::open(&path).expect("Couldn't open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut by_line: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(_) => by_line.push(line.unwrap()),
            Err(_) => break,
        }
    }

    by_line
}

fn split_to_int_vecs (vec: Vec<String>) -> Vec<Vec<i64>> {
    let mut int_vecs: Vec<Vec<i64>> = Vec::new();

    for line in vec {
        let split: Vec<&str> = line.split(" ").collect();
        let mut int_vec: Vec<i64> = Vec::new();
        for s in split {
            let i = string_to_int(s);
            int_vec.push(i);
        }
        int_vecs.push(int_vec);
    }

    int_vecs
}

fn string_to_int (s: &str) -> i64 {

    let int: i64 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Fuck, that string didn't become an int"),
    };

    int
}

fn check_safety (report: &Vec<i64>) -> bool {
    let ascending: bool = report[0] < report[1];
    
    if ascending {
        for i in 1..report.len() {
            let diff = (report[i - 1] - report[i]).abs();
            if report[i - 1] > report[i] || 3 < diff || diff < 1  {
                return false
            }
        }
    } else {
        for i in 1..report.len() {
            let diff = (report[i - 1] - report[i]).abs();
            if report[i - 1] < report[i] || 3 < diff || diff < 1  {
                return false
            }
        }
    }

    true
}

fn check_dampened_safety (report: &Vec<i64>) -> bool {

    for i in 0..report.len() {
        let mut test = report.clone();
        test.remove(i);
        if check_safety(&test) {
            return true
        }
    }

    false
}