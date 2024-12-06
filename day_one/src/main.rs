use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let path: &str = "./input.txt";

    let file: File = File::open(&path).expect("Couldn't open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut by_line: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(_) => by_line.push(line.unwrap()),
            Err(_) => break,
        }
    }

    let mut locations_one: Vec<i64> = Vec::new();
    let mut locations_two: Vec<i64> = Vec::new();
    for line in by_line {
        let split: Vec<&str> = line.split("   ").collect();
        locations_one.push(string_to_int(&split[0]));
        locations_two.push(string_to_int(&split[1]));
    }

    locations_one.sort();
    locations_two.sort();

    let mut distances: Vec<i64> = Vec::new();
    for i in 0..locations_one.len() {
        distances.push(
            (locations_one[i] - locations_two[i]).abs()
        );
    }

    let sum: i64 = distances.iter().sum();

    println!("The total distance is: {}", sum);

}

fn string_to_int (s: &str) -> i64 {
    let guess: i64 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Fuck, that didn't work"),
    };
    guess
}