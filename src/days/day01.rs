use std::fs;

pub fn solve(input_file: &str) {
    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| String::from(l))
        .collect();

    let int_vecs: (Vec<i32>, Vec<i32>) = split_to_int_vecs(by_line);
    let locations_one = int_vecs.0;
    let locations_two = int_vecs.1;

    let mut distances: Vec<i32> = Vec::new();
    for i in 0..locations_one.len() {
        distances.push((locations_one[i] - locations_two[i]).abs());
    }

    let sum: i32 = distances.iter().sum();

    println!("Total distance: {}", sum);

    let mut similarity_score: i32 = 0;
    for elem_1 in &locations_one {
        let mut sim_score: i32 = 0;
        for elem_2 in &locations_two {
            if elem_1 == elem_2 {
                sim_score += 1
            }
        }
        similarity_score += sim_score * elem_1
    }

    println!("Similarity score: {}", similarity_score)
}

fn split_to_int_vecs(vec: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut locations_one: Vec<i32> = Vec::new();
    let mut locations_two: Vec<i32> = Vec::new();

    for line in vec {
        let split: Vec<&str> = line.split("   ").collect();
        locations_one.push(split[0].trim().parse::<i32>().unwrap());
        locations_two.push(split[1].trim().parse::<i32>().unwrap());
    }

    locations_one.sort();
    locations_two.sort();

    (locations_one, locations_two)
}
