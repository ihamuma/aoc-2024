use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);
    
    let mut matrix = str_vec_to_matrix(by_line);
    let len_y = matrix.len() as i16;
    let len_x = matrix[0].len() as i16;
    
    let starting_position = match find_char(&matrix, '^') {
        Some(loc) => loc,
        None => panic!("Starting position not found in matrix!")
    };
    
    matrix[starting_position.0][starting_position.1] = 'X';

    let mut i = starting_position.0 as i16;
    let mut j = starting_position.1 as i16;
    let mut dir = "up";
    let mut positions: u16 = 1;

    loop {
        if      dir == "up"     { i -= 1 } 
        else if dir == "down"   { i += 1 }
        else if dir == "left"   { j -= 1 }
        else if dir == "right"  { j += 1 }

        if is_outside((&i, &j), &len_y, &len_x) { break }

        let y = i as usize;
        let x = j as usize;
        let next = matrix[y][x];

        if next == '#' { 
            if      dir == "up"     { i += 1 } 
            else if dir == "down"   { i -= 1 }
            else if dir == "left"   { j += 1 }
            else if dir == "right"  { j -= 1 }
            dir = turn_right(dir);
            continue;
        } else if next == 'X' {
            continue;
        } else {
            matrix[y][x] = 'X';
            positions += 1;
            continue;
        }

    }
    println!("The guard visits {positions} distinct positions before leaving the area")
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

fn str_vec_to_matrix(str_vec: Vec<String>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in str_vec {
        let as_char_vec: Vec<char> = line.chars().collect();
        matrix.push(as_char_vec);
    }
    matrix
}

fn find_char(matrix: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == target) {
            return Some((i, j));
        }
    }
    None
}

fn is_outside(pos: (&i16, &i16), len_y: &i16, len_x: &i16) -> bool {
    let y = pos.0;
    let x = pos.1;
    if *y < 0 || len_y <= y || 
       *x < 0 || len_x <= x {
        return true
    }
    false
}

fn turn_right(dir: &str) -> &str {
    if      dir == "up"     { return "right"}
    else if dir == "right"  { return "down" }
    else if dir == "down"   { return "left" }
    else                    { return "up" }
}