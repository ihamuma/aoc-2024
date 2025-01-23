use std::fs;

fn main() {
    let disk_map: Vec<u8> = fs::read_to_string("./input.txt")
        .unwrap()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let expanded_layout = expand_layout(disk_map);

    // Cloning to allow pt. 2 manipulations
    let mut expanded_layout_two = expanded_layout.clone();

    let empty_count = expanded_layout.iter().filter(|&n| *n == -1).count();

    let compacted_layout = compact_layout(expanded_layout, empty_count);

    let mut check_sum = 0;
    for (i, mem) in compacted_layout.into_iter().enumerate() {
        check_sum += i as i64 * mem as i64
    }

    println!("The initial check sum is {check_sum}");

    let mut free_space_locations = locate_free_spaces(&expanded_layout_two);
    let mut file_locations = locate_files(&expanded_layout_two);

    while !file_locations.is_empty() {
        let file_to_move = file_locations.pop().unwrap();
        let memory_need = file_to_move.size;
        let file_lower = file_to_move.lower;

        // Find leftmost adequate free space. Break at first found or if to right of file.
        let mut idx = 0;
        let mut space_found = false;
        for (i, fs) in free_space_locations.iter().enumerate() {
            if fs.lower > file_lower {
                break;
            } else if fs.size >= memory_need {
                idx = i;
                space_found = true;
                break;
            }
        }

        if !space_found {
            continue;
        }

        let mut free_space = free_space_locations.remove(idx);

        // If more free space than file needs, insert remaining space back
        if free_space.size > memory_need {
            let diff = free_space.size - memory_need;
            free_space_locations.insert(
                idx,
                FreeSpaceInfo {
                    size: diff,
                    lower: free_space.upper - (diff - 1),
                    upper: free_space.upper,
                },
            );
            free_space.upper = free_space.upper - diff;
        };

        let free_lower = free_space.lower;
        let free_upper = free_space.upper;

        let (left, right) = expanded_layout_two.split_at_mut(file_lower);

        if left.len() < memory_need {
            continue;
        }

        let file_to_move = &mut right[..memory_need];
        let space_for_file = &mut left[free_lower..=free_upper];

        space_for_file.swap_with_slice(file_to_move);
    }

    let mut second_check_sum = 0;
    for (i, mem) in expanded_layout_two.into_iter().enumerate() {
        if mem != -1 {
            second_check_sum += i as i64 * mem as i64
        }
    }

    println!("The final check sum is {second_check_sum}")
}

fn expand_layout(disk: Vec<u8>) -> Vec<i16> {
    let mut expanded_layout = Vec::new();
    let mut id: i16 = -1;
    for (i, mem) in disk.into_iter().enumerate() {
        if mem == 0 {
            continue;
        };

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
    expanded_layout
}

fn compact_layout(mut expanded: Vec<i16>, empties: usize) -> Vec<i16> {
    let mut compacted_layout = Vec::new();

    for i in 0..expanded.len() - empties {
        if expanded[i] == -1 {
            let mut last = expanded.pop().unwrap();
            while last == -1 {
                last = expanded.pop().unwrap();
            }
            compacted_layout.push(last);
        } else {
            compacted_layout.push(expanded[i]);
        }
    }
    compacted_layout
}

struct FreeSpaceInfo {
    size: usize,
    lower: usize,
    upper: usize,
}

// See locate_files for enumerating version
fn locate_free_spaces(memory: &Vec<i16>) -> Vec<FreeSpaceInfo> {
    let mut locations = Vec::new();
    let lim = memory.len() - 1;

    let mut i = 0;
    loop {
        while memory[i] != -1 {
            i += 1;
            if i >= lim {
                break;
            }
        }
        let lower = i;

        while memory[i] == -1 {
            i += 1;
            if i >= lim {
                break;
            }
        }
        let upper = i - 1;

        let size = upper + 1 - lower;
        let location = FreeSpaceInfo { size, lower, upper };

        locations.push(location);

        if i >= lim {
            break;
        }
    }

    locations
}

struct FileInfo {
    size: usize,
    lower: usize,
}

// See locate_free_files for non-enumerating version
fn locate_files(memory: &Vec<i16>) -> Vec<FileInfo> {
    let mut location_vec = Vec::new();
    let mut cur_id = memory[0];
    let mut start = 0;

    for (i, id) in memory.iter().enumerate() {
        if *id != cur_id {
            if cur_id != -1 {
                location_vec.push(FileInfo {
                    size: i - start,
                    lower: start,
                });
            }
            cur_id = *id;
            start = i;
        }
    }

    // Add last file to location vec if not in free space
    if cur_id != -1 {
        location_vec.push(FileInfo {
            size: memory.len() - start,
            lower: start,
        })
    };

    location_vec
}
