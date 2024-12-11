use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::RegexSet;

fn main() {
    let path: &str = "./test_input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);

    let horizontal_matches = count_matches(&by_line);
    println!("Horizontal matches: {}", horizontal_matches);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in by_line {
        let as_char_vec: Vec<char> = line.chars().collect();
        matrix.push(as_char_vec);
    }
    
    let transposed = transpose(matrix);
    let string_vec = char_to_string_vec(transposed);
    let vertical_matches = count_matches(&string_vec);
    println!("Vertical matches: {}", vertical_matches);

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

fn count_matches (matchable: &Vec<String>) -> u32 {
    let set = RegexSet::new(&[
        r"XMAS",
        r"SAMX",
    ]).unwrap();

    let mut matches = Vec::new();
    for s in matchable {
        let m: Vec<_> = set.matches(&s).into_iter().collect();
        matches.push(m);
    }

    let flat_matches = flatten(matches);
    let counted_matches: u32 = flat_matches.len().try_into().unwrap();
    
    counted_matches
}

fn flatten<T> (nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

fn transpose (v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn char_to_string_vec (char_vec: Vec<Vec<char>>) -> Vec<String> {
    let mut string_vec = Vec::new();
    for tsd in char_vec {
        let to_string: String = tsd.iter().collect();
        string_vec.push(to_string)
    }
    string_vec
}

