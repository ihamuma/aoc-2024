use std::fs;

fn main() {
    let disk_map: Vec<u8> = fs::read_to_string("./input.txt").unwrap()
                                                                    .chars()
                                                                    .collect::<Vec<char>>()
                                                                    .into_iter()
                                                                    .map(|ch|ch.to_digit(10).unwrap() as u8)
                                                                    .collect();

    let mut expanded_layout = Vec::new();
    let mut id: i16 = -1;
    for (i, mem) in disk_map.into_iter().enumerate() {
        if mem == 0 { continue };

        if i % 2 == 0 {
            id += 1;
            for _ in 0..mem {
                expanded_layout.push(id);
            }
        } else {
            for _ in 0..mem {
                expanded_layout.push(-1);
            }
        }
    }

    let empty_count = expanded_layout.iter()
                                            .filter(|&n| *n == -1)
                                            .count();

    let mut compacted_layout = Vec::new();

    for i in 0..expanded_layout.len() - empty_count {
        if expanded_layout[i] == -1 {
            let mut last = expanded_layout.pop().unwrap();
            while last == -1 {
                last = expanded_layout.pop().unwrap();
            }
            compacted_layout.push(last);
        } else {
            compacted_layout.push(expanded_layout[i]);
        }
    }

    let mut check_sum = 0;
    for (i, mem) in compacted_layout.into_iter().enumerate() {
        check_sum += i as i64 * mem as i64
    }

    println!("The check sum is {check_sum}");
}
