use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 && args[1] == "test" {
        "day_3/test_input.txt"
    } else {
        "day_3/input.txt"
    };

    let corrupted_memory: String = fs::read_to_string(input_file).expect("File read not OK");

    let to_multiply = regex_for_multipliables(&corrupted_memory);
    let sum = sum_multiples_from_str_vec(to_multiply);

    println!("The sum of uncorrupted multiplications is {}", sum);

    let split_dos = split_by_do_and_dont(&corrupted_memory);
    let joined_dos = split_dos.join("");
    let dos_to_multiply = regex_for_multipliables(&joined_dos);
    let summed_dos = sum_multiples_from_str_vec(dos_to_multiply);

    println!(
        "The sum of uncorrupted multiplications to 'do' is {}",
        summed_dos
    );
}

fn regex_for_multipliables(matchable: &str) -> Vec<(&str, &str)> {
    let re = Regex::new(r"(mul\((?<m1>\d+),(?<m2>\d+)\))").unwrap();
    let multipliables: Vec<(&str, &str)> = re
        .captures_iter(matchable)
        .map(|caps| {
            let mult_1 = caps.name("m1").unwrap().as_str();
            let mult_2 = caps.name("m2").unwrap().as_str();
            (mult_1, mult_2)
        })
        .collect();
    multipliables
}

fn sum_multiples_from_str_vec(str_vec: Vec<(&str, &str)>) -> u32 {
    let mut sum = 0;

    for m in str_vec {
        let m1 = string_to_int(m.0);
        let m2 = string_to_int(m.1);
        sum += m1 * m2
    }

    sum
}

fn string_to_int(s: &str) -> u32 {
    let int: u32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Fuck, that didn't work"),
    };

    int
}

fn split_by_do_and_dont(memory: &str) -> Vec<&str> {
    let mut split_by_dont: Vec<&str> = memory.split("don't()").collect();
    // Store first split, as contains strings to do
    let first_dos = split_by_dont[0];
    split_by_dont.remove(0);

    let mut split_by_do: Vec<Vec<&str>> = Vec::new();
    for s in &split_by_dont {
        let mut split: Vec<&str> = s.split("do()").collect();
        // Due to first split by don't, can remove first index (=not-doable strings)
        split.remove(0);
        split_by_do.push(split);
    }

    let mut flat_dos = flatten(split_by_do);
    flat_dos.push(first_dos);

    flat_dos
}

fn flatten(nested: Vec<Vec<&str>>) -> Vec<&str> {
    nested.into_iter().flatten().collect()
}
