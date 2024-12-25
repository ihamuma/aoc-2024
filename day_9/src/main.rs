use indicatif::{ProgressBar, ProgressIterator};
use std::fs;

fn main() {
    let disk_map: Vec<u8> = fs::read_to_string("./input.txt").unwrap()
                                                                .chars()
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

    // Cloning to allow pt. 2 manipulations
    let mut expanded_layout_two = expanded_layout.clone();

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
    };

    let mut check_sum = 0;
    for (i, mem) in compacted_layout.into_iter().enumerate() {
        check_sum += i as i64 * mem as i64
    }

    println!("The initial check sum is {check_sum}");

    let mut i = expanded_layout_two.len() - 1;
    //println!("{:?}", &expanded_layout_two[..9]);

    let bar_size = (expanded_layout_two.len() - 2) as u64;
    let bar = ProgressBar::new(bar_size);

    // First part of memory is always occupied, no need to check last position
    while i > 0 {
        // Current element to examine
        let cur_id = expanded_layout_two[i];

        // While next id == previous id, iterate until changes
        let mut j = i-1;
        while expanded_layout_two[j] == cur_id && j > 0 {
            bar.inc(1);
            j -= 1;
        }
        // Add one so as to not take unmatching id
        let lower_bound = j+1;
        let memory_need = i-j;

        // Slice layout at low bound to enable mutation
        let (left, right) = expanded_layout_two.split_at_mut(lower_bound);

        // Create slice based on lowest & highest 
        let movable = &mut right[..memory_need];

        let mut allocatable = false;
        let mut lower_free = 0;
        let mut upper_free = 0+memory_need;

        if left.len() >= memory_need {
            for k in 0..left.len()-memory_need {
                let test_slice = &mut left[k..k+memory_need];
                if test_slice.iter().all(|&x| x == -1) {
                    lower_free = k;
                    upper_free = k+memory_need;
                    allocatable = true;
                    break
                }
            }
        }

        if allocatable {
            let allocated = &mut left[lower_free..upper_free];
            allocated.swap_with_slice(movable);
            //println!("{:?}", &expanded_layout_two[..19]);
        }

        i = j;
    }

    bar.finish();

    let mut second_check_sum = 0;
    for (i, mem) in expanded_layout_two.into_iter().enumerate().progress() {
        if mem != -1 {
            second_check_sum += i as i64 * mem as i64
        }
    }

    println!("The final check sum is {second_check_sum}")
    // Current too high 6390782022205

}
