use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in by_line {
        let as_char_vecs: Vec<char> = line.chars().collect();
        matrix.push(as_char_vecs);
    }

    let xmases = count_xmases(&matrix);
    println!("The amount of xmases is {}", xmases);

    let xed_mases = count_xed_mases(&matrix);
    println!("The amount of X-MASes is {}", xed_mases)
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

fn count_xmases (matrix: &Vec<Vec<char>>) -> u32 {
    let mut xmas_count: u32 = 0;
    let xmas = String::from("XMAS");

    // TODO: Fix this absolutely horrid solution
    let e_m = expand_matrix(&matrix);

    for i in 3..e_m.len()-3 {
        for j in 3..e_m[0].len()-3 {
            if e_m[i][j] == 'X' {
                let left =  vec!['X', e_m[i][j-1], e_m[i][j-2], e_m[i][j-3]];
                let right = vec!['X', e_m[i][j+1], e_m[i][j+2], e_m[i][j+3]];
                let up =    vec!['X', e_m[i-1][j], e_m[i-2][j], e_m[i-3][j]];
                let down =  vec!['X', e_m[i+1][j], e_m[i+2][j], e_m[i+3][j]];
                let up_left =       vec!['X', e_m[i-1][j-1], e_m[i-2][j-2], e_m[i-3][j-3]];
                let up_right =      vec!['X', e_m[i-1][j+1], e_m[i-2][j+2], e_m[i-3][j+3]];
                let down_left =     vec!['X', e_m[i+1][j-1], e_m[i+2][j-2], e_m[i+3][j-3]];
                let down_right =    vec!['X', e_m[i+1][j+1], e_m[i+2][j+2], e_m[i+3][j+3]];

                let maybe_xmases = vec![left, right, up, down, up_left, up_right, down_left, down_right];

                for maybe in maybe_xmases {
                    let stringed: String = maybe.iter().collect();
                    if stringed == xmas {
                        xmas_count += 1
                    }
                }
            }
        }
    }

    xmas_count
}

fn expand_matrix (matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = Vec::new();
    let padding = vec!['A'; matrix.len()+6];
    let triple_a = vec!['A', 'A', 'A'];
    
    for _ in 0..3 {expanded.push(padding.clone())};

    for line in matrix {
        let mut new_line: Vec<char> = Vec::new();
        new_line.extend(&triple_a);
        new_line.extend(line);
        new_line.extend(&triple_a);
        expanded.push(new_line);
    }

    for _ in 0..3 {expanded.push(padding.clone())};

    expanded
}

fn count_xed_mases (matrix: &Vec<Vec<char>>) -> u32 {
    let mut xed_mas_count: u32 = 0;
    let mas = String::from("MAS");
    let sam = String::from("SAM");

    for i in 1..matrix.len()-1 {
        for j in 1..matrix[0].len()-1 {
            if matrix[i][j] == 'A' {
                let diag_1 = vec![matrix[i-1][j-1], 'A', matrix[i+1][j+1]];
                let diag_2 = vec![matrix[i+1][j-1], 'A', matrix[i-1][j+1]];
                let diag_1: String = diag_1.iter().collect();
                let diag_2: String = diag_2.iter().collect();
                if (diag_1 == mas || diag_1 == sam ) && (diag_2 == mas || diag_2 == sam) {
                    xed_mas_count += 1;
                }
            }
        }
    }
    xed_mas_count
}