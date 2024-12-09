use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";

    let by_line: Vec<String> = file_to_string_vec(path);

    let reports: Vec<Vec<i64>> = split_to_int_vecs(by_line);

    let mut safes: u32 = 0;

    for report in reports {
        if check_safety(&report) {
            safes += 1
        }
    }

    println!("There are {} safe reports", safes)
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
