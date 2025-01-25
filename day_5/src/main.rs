use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 && args[1] == "test" {
        "day_5/test_input.txt"
    } else {
        "day_5/input.txt"
    };
    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| String::from(line))
        .collect();

    let (rules, updates) = extract_rules_and_updates(&by_line);

    let mapped_rules = tuples_to_hashmap(&rules);

    let mut valid_mid_elem_sum = 0;
    let mut invalid_updates = Vec::new();
    for update in updates {
        if update_is_valid(&update, &mapped_rules) {
            valid_mid_elem_sum += middle_element(&update)
        } else {
            invalid_updates.push(update);
        }
    }
    println!("The sum of all valid middle elements is {valid_mid_elem_sum}");

    let mut fixed_mid_elem_sum = 0;
    for mut inv in invalid_updates {
        let fixed = fix_invalid_update(&mut inv, &mapped_rules);
        fixed_mid_elem_sum += middle_element(&fixed)
    }
    println!("The sum of all fixed middle elements is {fixed_mid_elem_sum}");
}

fn extract_rules_and_updates(r_and_u: &Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in r_and_u {
        if line.contains("|") {
            if let Some((p1, p2)) = line.split_once('|') {
                rules.push((string_to_int(p1), string_to_int(p2)));
            } else {
                println!("{} didn't compile to a rule", line)
            }
        }
        if line.contains(",") {
            let split: Vec<u32> = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
            updates.push(split);
        }
    }
    (rules, updates)
}
fn string_to_int(s: &str) -> u32 {
    let int: u32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Fuck, that string didn't become an int"),
    };

    int
}

fn tuples_to_hashmap(tup_vec: &Vec<(u32, u32)>) -> HashMap<u32, HashSet<u32>> {
    let mut map = HashMap::new();
    for tup in tup_vec {
        map.entry(tup.0).or_insert(HashSet::new()).insert(tup.1);
    }
    map
}

fn update_is_valid(update: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 1..update.len() {
        let rule_set = match rules.get(&update[i]) {
            Some(set) => set,
            None => continue,
        };
        for j in 0..i {
            if rule_set.contains(&update[j]) {
                return false;
            }
        }
    }
    true
}

fn middle_element(vec: &Vec<u32>) -> u32 {
    vec[vec.len() / 2]
}

fn fix_invalid_update(update: &mut Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    while !update_is_valid(&update, rules) {
        for i in 1..update.len() {
            let rule_set = match rules.get(&update[i]) {
                Some(set) => set,
                None => continue,
            };
            for j in 0..i {
                if rule_set.contains(&update[j]) {
                    update.swap(i, j);
                }
            }
        }
    }
    let res = update.to_vec();
    res
}
