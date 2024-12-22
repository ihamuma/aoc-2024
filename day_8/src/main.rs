fn main() {
    let input = std::fs::read_to_string("./test_input.txt").unwrap();

    let matrix: Vec<Vec<char>> = input.lines()
                      .map(|line| line.chars().collect())
                      .collect();
    
    let deviant_char_positions = find_deviant_chars(&matrix, '.');
    
    let antennae: Vec<Antenna> = deviant_char_positions.iter()
                                         .map(|pos| Antenna::from(pos))
                                         .collect();

    println!("{antennae:#?}")
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    row: usize,
    col: usize
}

impl Antenna {
    fn new () -> Antenna {
        Antenna {
            frequency: 'a',
            row: 0,
            col: 0
        }
    }

    fn from (char_pos: &(char, usize, usize)) -> Antenna {
        let mut antenna = Antenna::new();
        
        antenna.frequency = char_pos.0;
        antenna.row       = char_pos.1;
        antenna.col       = char_pos.2;

        antenna
    }

    // Compare with other Antenna. If same frequency, create antinodes in both directions.
    // Antinodes (and maybe antennae?) should all be i8, not usize, to enable possible negative values
    // Once created, Antinodes should have validate-function for checking if they are inside the bounds of the area
    //  Namely: if row < 0 || col < 0 || row >= matrix.len() || col >= matrix.len() {return false}
    fn compare (&self, other: &Antenna) -> Some((Antinode, Antinode)) {
        if self.frequency != other.frequency {
            return None
        }

        row_diff = (self.row as i8 - other.row as i8).abs();
        col_diff = (self.col as i8 - other.col as i8).abs();
    }
}

#[derive(Debug)]
struct Antinode {
    row: usize, 
    col: usize,
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