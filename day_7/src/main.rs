use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);

    let mut sum_calculation_map = HashMap::new();
    for line in &by_line {
        if let Some((sum, to_calculate)) = line.split_once(": ") {
            let sum = sum.parse::<u128>().unwrap();
            let calc_vec: Vec<u128> = to_calculate.split(" ")
                                        .map(|x| x.parse::<u128>().unwrap())
                                        .collect();
            
            if sum_calculation_map.contains_key(&sum) {
                panic!("{sum} exists at least twice")
            } else {
                sum_calculation_map.insert(sum, calc_vec);
            }
        }
    }

    for (key, value) in &sum_calculation_map {
        println!("{key}: {value:?}");
    }
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