use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path: &str = "./input.txt";

    let by_line: Vec<String> = file_to_string_vec(path);

    let int_vecs: (Vec<i64>, Vec<i64>) = split_to_int_vecs(by_line);
    let locations_one = int_vecs.0;
    let locations_two = int_vecs.1;

    let mut distances: Vec<i64> = Vec::new();
    for i in 0..locations_one.len() {
        distances.push((locations_one[i] - locations_two[i]).abs());
    }

    let sum: i64 = distances.iter().sum();

    println!("The total distance is: {}", sum);

    let mut similarity_score: i64 = 0;
    for elem_1 in &locations_one {
        let mut sim_score: i64 = 0;
        for elem_2 in &locations_two {
            if elem_1 == elem_2 {
                sim_score += 1
            }
        }
        similarity_score += sim_score * elem_1
    }

    println!("The similarity score is: {}", similarity_score)
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

fn split_to_int_vecs(vec: Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut locations_one: Vec<i64> = Vec::new();
    let mut locations_two: Vec<i64> = Vec::new();

    for line in vec {
        let split: Vec<&str> = line.split("   ").collect();
        locations_one.push(string_to_int(&split[0]));
        locations_two.push(string_to_int(&split[1]));
    }

    locations_one.sort();
    locations_two.sort();

    (locations_one, locations_two)
}

fn string_to_int(s: &str) -> i64 {
    let guess: i64 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Fuck, that didn't work"),
    };

    guess
}
