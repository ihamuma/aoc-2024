use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let row_bound = matrix.len() as i8;
    let col_bound = matrix[0].len() as i8;

    let deviant_char_positions = find_deviant_chars(&matrix, '.');

    let antennae: Vec<Antenna> = deviant_char_positions
        .iter()
        .map(|pos| Antenna::from(pos))
        .collect();

    let mut antinode_set = HashSet::new();

    for i in 0..antennae.len() - 1 {
        let antenna_1 = &antennae[i];

        for j in i + 1..antennae.len() {
            let antenna_2 = &antennae[j];

            if let Some(antinode_vec) =
                Antenna::compare_for_nodes(&antenna_1, &antenna_2, row_bound, col_bound)
            {
                for node in antinode_vec {
                    antinode_set.insert(node);
                }
            }
        }
    }

    println!("There are {} unique antinodes", antinode_set.len())
}

fn find_deviant_chars(matrix: &Vec<Vec<char>>, norm: char) -> Vec<(char, usize, usize)> {
    let mut results = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != norm {
                results.push((ch, i, j));
            }
        }
    }

    results
}

struct Antenna {
    frequency: char,
    row: i8,
    col: i8,
}

impl Antenna {
    fn new() -> Antenna {
        Antenna {
            frequency: 'a',
            row: 0,
            col: 0,
        }
    }

    fn from(char_pos: &(char, usize, usize)) -> Antenna {
        let mut antenna = Antenna::new();

        antenna.frequency = char_pos.0;
        antenna.row = char_pos.1 as i8;
        antenna.col = char_pos.2 as i8;

        antenna
    }

    // Compare with other Antenna. If same frequency, create antinodes in both directions.
    fn compare_for_nodes(
        &self,
        other: &Antenna,
        row_bound: i8,
        col_bound: i8,
    ) -> Option<Vec<Antinode>> {
        if self.frequency != other.frequency {
            return None;
        }

        let row_diff = self.row - other.row;
        let col_diff = self.col - other.col;

        let mut antinode_vec = Vec::new();

        let mut valid_antinode = true;

        antinode_vec.push(Antinode {
            row: self.row,
            col: self.col,
        });

        while valid_antinode {
            let previous_node = antinode_vec.last().unwrap();

            let self_antinode = Antinode {
                row: previous_node.row + row_diff,
                col: previous_node.col + col_diff,
            };

            if Antinode::validate(&self_antinode, row_bound, col_bound) {
                antinode_vec.push(self_antinode);
            } else {
                valid_antinode = false
            }
        }

        valid_antinode = true;

        antinode_vec.push(Antinode {
            row: other.row,
            col: other.col,
        });

        while valid_antinode {
            let previous_node = antinode_vec.last().unwrap();

            let other_antinode = Antinode {
                row: previous_node.row - row_diff,
                col: previous_node.col - col_diff,
            };

            if Antinode::validate(&other_antinode, row_bound, col_bound) {
                antinode_vec.push(other_antinode);
            } else {
                valid_antinode = false
            }
        }

        Some(antinode_vec)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Antinode {
    row: i8,
    col: i8,
}

impl Antinode {
    fn validate(&self, row_bound: i8, col_bound: i8) -> bool {
        self.row >= 0 && self.row < row_bound && self.col >= 0 && self.col < col_bound
    }
}
