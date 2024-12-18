use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path: &str = "./input.txt";
    let by_line: Vec<String> = file_to_string_vec(path);
    
    let mut matrix: Vec<Vec<char>> = str_vec_to_matrix(by_line);
    let len_y = matrix.len() as i16;
    let len_x = matrix[0].len() as i16;
    
    let starting_position = match find_char(&matrix, '^') {
        Some(pos) => pos,
        None => panic!("Starting position not found in matrix!")
    };

    // Mark starting position as already visited, avoid double counting
    matrix[starting_position.0][starting_position.1] = 'X';

    // Cast as i16 to allow checking for negative values
    let mut i = starting_position.0 as i16;
    let mut j = starting_position.1 as i16;
    let mut dir = "up";
    let mut positions: u16 = 1;

    loop {
        let new_pos = move_in_dir(dir, i, j);
        i = new_pos.0;
        j = new_pos.1;

        if is_outside((&i, &j), &len_y, &len_x) { break }

        // If !is_outside, both i & j are positive and can be cast to usize to get next char
        let y = i as usize;
        let x = j as usize;
        let next = matrix[y][x];

        // If next is an obstacle, go back to previous and turn right
        if next == '#' { 
            let prev_pos = move_back(dir, i, j);
            i = prev_pos.0;
            j = prev_pos.1;
            dir = turn_right(dir);
            continue;
        // If next has been previously visited, ignore
        } else if next == 'X' {
            continue;
        // Else, mark as visited, add one to positions count and continue to next position.
        } else {
            matrix[y][x] = 'X';
            positions += 1;
            continue;
        }
    }

    println!("The guard visits {positions} distinct positions before leaving the area");

    // Reset: mark starting as P to signal previously checked obstacle position
    matrix[starting_position.0][starting_position.1] = 'P';
    i = starting_position.0 as i16;
    j = starting_position.1 as i16;
    dir = "up";

    // Create separate iterators for obstacle placement
    let mut obs_i = i;
    let mut obs_j = j;
    let mut obs_dir = dir;
    let mut looping_obstacles: u16 = 0;
    
    'obstacle_placing: loop {
        // Place obstacle according to previous position and direction
        let new_obs_pos = move_in_dir(obs_dir, obs_i, obs_j);
        obs_i = new_obs_pos.0;
        obs_j = new_obs_pos.1;

        if is_outside((&obs_i, &obs_j), &len_y, &len_x) { break 'obstacle_placing; }

        let obs_y = obs_i as usize;
        let obs_x = obs_j as usize;
        let next_obs = matrix[obs_y][obs_x];

        // If next obstacle placement is an original obstacle, go back to previous and turn right
        if next_obs == '#' {
            let prev_obs_pos = move_back(obs_dir, obs_i, obs_j);
            obs_i = prev_obs_pos.0;
            obs_j = prev_obs_pos.1;
            obs_dir = turn_right(&obs_dir);
            continue;
        // If next has previously been checked for looping, continue
        } else if next_obs == 'P' {
            continue
        // Else place new obstacle
        } else {
            matrix[obs_y][obs_x] = 'O'
        }

        // Place "test guard" in position before new obstacle
        let new_start_pos = move_back(&obs_dir, obs_i, obs_j);
        i = new_start_pos.0;
        j = new_start_pos.1;

        // Initialise HashSet to store previous locations
        let mut location_map = HashSet::new();

        // Turn right to not hit new obstacle immediately
        dir = turn_right(&obs_dir);

        'loop_or_exit: loop {
            // Move to next position
            let new_pos = move_in_dir(dir, i, j);
            i = new_pos.0;
            j = new_pos.1;
    
            // If guard makes it outside, mark as previous obstacled and break loop
            if is_outside((&i, &j), &len_y, &len_x) { 
                matrix[obs_y][obs_x] = 'P';
                break 'loop_or_exit; 
            }

            // If same dir, location pair already in location_map
            // -> second time to that location with same dir
            // -> guard looped, add to looped count, break loop.
            if location_map.contains(&(dir, (i, j))) { 
                looping_obstacles += 1;
                matrix[obs_y][obs_x] = 'P';
                break 'loop_or_exit;
            } else {
                location_map.insert((dir, (i, j)));
            }
    
            let y = i as usize;
            let x = j as usize;
            let next = matrix[y][x];

            if next == '#' || next == 'O' { 
                let prev_pos = move_back(dir, i, j);
                i = prev_pos.0;
                j = prev_pos.1;
                dir = turn_right(dir);
                continue;
            }

        }
    }

    println!("There are {looping_obstacles} ways to place an object and create an infinite loop")
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

fn move_in_dir(dir: &str, i: i16, j: i16) -> (i16, i16) {
    if      dir == "up"     { return (i-1, j) } 
    else if dir == "down"   { return (i+1, j) }
    else if dir == "left"   { return (i, j-1) }
    else                    { return (i, j+1)}
}

fn move_back(dir: &str, i: i16, j: i16) -> (i16, i16) {
    if      dir == "up"     { return (i+1, j) } 
    else if dir == "down"   { return (i-1, j) }
    else if dir == "left"   { return (i, j+1) }
    else                    { return (i, j-1) }
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