use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", start_position(&input, 4));
    println!("Part 2: {}", start_position(&input, 14));
}

fn start_position(input: &str, distinct_chars: usize) -> u32 {
    for (i, _) in input.chars().enumerate() {
        if i < distinct_chars {
            continue;
        }
        let chars: HashSet<char> = input[i-distinct_chars..i].chars().collect();
        if chars.len() == distinct_chars {
            return i as u32;
        }
    };
    panic!("no packet start position found");
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}