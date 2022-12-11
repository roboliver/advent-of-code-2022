use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", misplaced_item_priority_sum(&input));
    println!("Part 2: {}", badge_priority_sum(&input));
}

fn misplaced_item_priority_sum(input: &str) -> u32 {
    input.lines()
        .map(misplaced_item)
        .map(item_priority)
        .sum()
}

fn badge_priority_sum(input: &str) -> u32 {
    let groups = elf_groups(input);
    groups.iter()
        .map(badge)
        .map(item_priority)
        .sum()
}

fn misplaced_item(rucksack: &str) -> char {
    let compartments = compartments(rucksack);
    let left_compartment: HashSet<char> = compartments.0.chars().collect();
    let right_compartment = compartments.1;
    right_compartment.chars()
        .filter(|item| left_compartment.contains(item))
        .next()
        .expect("no common items")
}

fn badge(group: &(&str, &str, &str)) -> char {
    let first_elf: HashSet<char> = group.0.chars().collect();
    let second_elf: HashSet<char> = group.1.chars().collect();
    let third_elf = group.2;
    third_elf.chars()
        .filter(|item| first_elf.contains(item))
        .filter(|item| second_elf.contains(item))
        .next()
        .expect("no common items")

}

fn item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - 96
    } else {
        (item as u32) - 38
    }
}

fn compartments(rucksack: &str) -> (&str, &str) {
    let midpoint = rucksack.len() / 2;
    (&rucksack[..midpoint], &rucksack[midpoint..])
}

fn elf_groups(input: &str) -> Vec<(&str, &str, &str)> {
    let mut groups: Vec<(&str, &str, &str)> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    for line in input.lines() {
        group.push(line);
        if group.len() == 3 {
            groups.push((group[0], group[1], group[2]));
            group = Vec::new();
        }
    }
    groups
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}