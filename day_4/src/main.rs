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
    
    let matrix_clone = matrix.clone();
    let transposed = transpose(matrix);
    let mut trans_clone = transposed.clone();
    let string_vec = char_to_string_vec(transposed);
    let vertical_matches = count_matches(&string_vec);
    println!("Vertical matches: {}", vertical_matches);

    for mcs in &matrix_clone {
        println!("{:?}", mcs)
    }
    let diagonals_1 = diagonal_matrix(matrix_clone);
    for diag in &diagonals_1 {
        println!("{:?}", diag)
    }
    let string_vec = char_to_string_vec(diagonals_1);
    let diagonal_1_matches = count_matches(&string_vec);
    println!("Diagonal 1 matches: {}", diagonal_1_matches);

    trans_clone.iter_mut().for_each(|arr| arr.reverse());
    for tsds in &trans_clone {
        println!("{:?}", tsds)
    }
    let diagonals_2 = diagonal_matrix(trans_clone);
    for diag in &diagonals_2 {
        println!("{:?}", diag)
    }
    let string_vec = char_to_string_vec(diagonals_2);
    let diagonal_2_matches = count_matches(&string_vec);
    println!("Diagonal 2 matches: {}", diagonal_2_matches);

    let total = horizontal_matches + vertical_matches + diagonal_1_matches + diagonal_2_matches;

    println!("Total matches: {}", total)
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

fn diagonal_matrix (matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len_y = matrix.len();
    let len_x = matrix[0].len();
    assert!(len_y == len_x);
    
    let mut diag_matr = Vec::new();
    for h in 4..=len_y {
        let mut low_vec = Vec::new();
        let mut high_vec = Vec::new();
        let mut i = len_y - h;
        let mut j = 0;
        while i != len_y {
            let push_low = matrix[i][j];
            let push_high = matrix[j][i];
            low_vec.push(push_low);
            high_vec.push(push_high);
            i += 1;
            j += 1;
        }
        diag_matr.push(low_vec);
        diag_matr.push(high_vec);
    }
    let _last = diag_matr.pop();

    diag_matr
}