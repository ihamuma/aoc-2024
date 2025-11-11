use std::fs;

pub fn solve(input_file: &str) {
    let matrix: Vec<Vec<char>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let xmases = count_xmases(&matrix);
    println!("xmases: {}", xmases);

    let xed_mases = count_xed_mases(&matrix);
    println!("X-MASes: {}", xed_mases)
}

fn count_xmases(matrix: &Vec<Vec<char>>) -> u32 {
    let mut xmas_count: u32 = 0;
    let xmas = String::from("XMAS");

    // TODO: Fix this absolutely horrid solution
    let e_m = expand_matrix(&matrix);

    for i in 3..e_m.len() - 3 {
        for j in 3..e_m[0].len() - 3 {
            if e_m[i][j] == 'X' {
                let left = vec!['X', e_m[i][j - 1], e_m[i][j - 2], e_m[i][j - 3]];
                let right = vec!['X', e_m[i][j + 1], e_m[i][j + 2], e_m[i][j + 3]];
                let up = vec!['X', e_m[i - 1][j], e_m[i - 2][j], e_m[i - 3][j]];
                let down = vec!['X', e_m[i + 1][j], e_m[i + 2][j], e_m[i + 3][j]];
                let up_left = vec!['X', e_m[i - 1][j - 1], e_m[i - 2][j - 2], e_m[i - 3][j - 3]];
                let up_right = vec!['X', e_m[i - 1][j + 1], e_m[i - 2][j + 2], e_m[i - 3][j + 3]];
                let down_left = vec!['X', e_m[i + 1][j - 1], e_m[i + 2][j - 2], e_m[i + 3][j - 3]];
                let down_right = vec!['X', e_m[i + 1][j + 1], e_m[i + 2][j + 2], e_m[i + 3][j + 3]];

                let maybe_xmases = vec![
                    left, right, up, down, up_left, up_right, down_left, down_right,
                ];

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

fn expand_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = Vec::new();
    let padding = vec!['A'; matrix.len() + 6];
    let triple_a = vec!['A', 'A', 'A'];

    for _ in 0..3 {
        expanded.push(padding.clone())
    }

    for line in matrix {
        let mut new_line: Vec<char> = Vec::new();
        new_line.extend(&triple_a);
        new_line.extend(line);
        new_line.extend(&triple_a);
        expanded.push(new_line);
    }

    for _ in 0..3 {
        expanded.push(padding.clone())
    }

    expanded
}

fn count_xed_mases(matrix: &Vec<Vec<char>>) -> u32 {
    let mut xed_mas_count: u32 = 0;
    let mas = String::from("MAS");
    let sam = String::from("SAM");

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] == 'A' {
                let diag_1 = vec![matrix[i - 1][j - 1], 'A', matrix[i + 1][j + 1]];
                let diag_2 = vec![matrix[i + 1][j - 1], 'A', matrix[i - 1][j + 1]];
                let diag_1: String = diag_1.iter().collect();
                let diag_2: String = diag_2.iter().collect();
                if (diag_1 == mas || diag_1 == sam) && (diag_2 == mas || diag_2 == sam) {
                    xed_mas_count += 1;
                }
            }
        }
    }
    xed_mas_count
}
