use std::fs;
use core::str;

fn main() {
    let input = read_input();
    println!("Part 1: {}", max_elf_calories(&input));
    println!("Part 2: {}", max_three_elf_calories(&input));
}

fn max_elf_calories(input: &str) -> u32 {
    calories_per_elf(&input)
        .into_iter()
        .max()
        .expect("No elves found")
}

fn max_three_elf_calories(input: &str) -> u32 {
    let mut calories = calories_per_elf(&input);
    calories.sort();
    calories.reverse();
    calories[..3]
        .iter()
        .sum()
}

fn calories_per_elf(input: &str) -> Vec<u32> {
    input.replace("\r", "")
        .split("\n\n")
        .map(|elf_foods| {
            elf_foods.lines()
                .map(|food| parse_calories(food))
                .sum()
        })
        .collect()
}

fn parse_calories(food: &str) -> u32 {
    food.trim().parse().expect("Not a number")
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}
