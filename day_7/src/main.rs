use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

// Current solution 434063141838 is too low
fn main() {
    let path: &str = "./input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);
    let sum_calculation_map = map_sums_and_calculations(by_line);

    let before = Instant::now();

    let mut i = 0;
    let mut valids_sum: u128 = 0;
    let mut invalid_keys: Vec<u128> = Vec::new();
    for (sum, calc) in sum_calculation_map {
        i += 1;
        println!("Evaluating {i} of 850");
        let product = calc.iter().fold(1, |res, a| res * a);
        if sum > product {
            //println!("Bigger: {sum} > {product}")
            invalid_keys.push(sum);
            continue
        }
        if sum == product {
            //println!("Equal: {sum} = {product}")
            valids_sum = valids_sum.checked_add(sum).expect("Overall sum variable overflowed!");
            continue
        }

        let all_options = evaluate_all(&calc);
        if all_options.contains(&sum) {
            valids_sum = valids_sum.checked_add(sum).expect("Overall sum variable overflowed!");
        } else {
            invalid_keys.push(sum);
        }

    }

    println!("The total calibration result is {valids_sum}");
    println!("There are {} invalid test values", invalid_keys.len());
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn file_to_string_vec (path: &str) -> Vec<String> {
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

fn map_sums_and_calculations (mappable: Vec<String>) -> HashMap<u128, Vec<u128>> {
    let mut map = HashMap::new();
    for line in &mappable {
        if let Some((sum, to_calculate)) = line.split_once(": ") {
            let sum = sum.parse::<u128>().unwrap();
            let calc_vec: Vec<u128> = to_calculate.split(" ")
                                        .map(|x| x.parse::<u128>().unwrap())
                                        .collect();            
            // Technically unsafe, but previously checked for duplicates.
            map.insert(sum, calc_vec);
        }
    }
    map
}

// This is dirty ChatGPT recursive code, made "safe" with overflow checks
fn evaluate_all(nums: &Vec<u128>) -> HashSet<u128> {
    let mut results = HashSet::new();

    if nums.len() == 1 {
        results.insert(nums[0]);
        return results;
    }

    for i in 1..nums.len() {
        let left = evaluate_all(&nums[0..i].to_vec());
        let right = evaluate_all(&nums[i..].to_vec());

        for &l in &left {
            for &r in &right {
                results.insert(l.checked_add(r).expect("{l} + {r} overflowed!"));
                results.insert(l.checked_mul(r).expect("{l} * {r} overflowed!"));
            }
        }
    }

    results
}