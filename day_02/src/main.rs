use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", total_score(&input));
    println!("Part 2: {}", total_score_actual(&input));
}

enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Lose,
    Draw,
    Win
}

fn total_score(input: &str) -> u32 {
    input.lines()
        .map(line_to_moves)
        .map(|moves| score(&moves.0, &moves.1))
        .sum()
}

fn total_score_actual(input: &str) -> u32 {
    input.lines()
        .map(line_to_move_and_goal)
        .map(|move_and_goal| {
            let my_move = my_move_for_outcome(&move_and_goal.0, &move_and_goal.1);
            (move_and_goal.0, my_move)
        })
        .map(|moves| score(&moves.0, &moves.1))
        .sum()
}

fn score(opp_move: &Move, my_move: &Move) -> u32 {
    shape_score(&my_move) + outcome_score(opp_move, my_move)
}

fn shape_score(a_move: &Move) -> u32 {
    match a_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn outcome_score(opp_move: &Move, my_move: &Move) -> u32 {
    match outcome(opp_move, my_move) {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn outcome(opp_move: &Move, my_move: &Move) -> Outcome {
    let opp_move_num = move_to_num(opp_move);
    let my_move_num = move_to_num(my_move) + 3;
    let outcome_num: u32 = (my_move_num - opp_move_num) % 3;
    num_to_outcome(outcome_num)
}

fn my_move_for_outcome(opp_move: &Move, my_goal: &Outcome) -> Move {
    let opp_move_num = move_to_num(opp_move);
    let my_goal_num = outcome_to_num(my_goal);
    let my_move_num = (opp_move_num + my_goal_num) % 3;
    num_to_move(my_move_num)
}

fn move_to_num(a_move: &Move) -> u32 {
    match a_move {
        Move::Scissors => 0,
        Move::Rock => 1,
        Move::Paper => 2
    }
}

fn num_to_move(a_move: u32) -> Move {
    match a_move {
        0 => Move::Scissors,
        1 => Move::Rock,
        2 => Move::Paper,
        _ => panic!("move number out of bounds: {}", a_move)
    }
}

fn outcome_to_num(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Draw => 0,
        Outcome::Win => 1,
        Outcome::Lose => 2
    }
}

fn num_to_outcome(outcome: u32) -> Outcome {
    match outcome {
        0 => Outcome::Draw,
        1 => Outcome::Win,
        2 => Outcome::Lose,
        _ => panic!("outcome number out of bounds: {}", outcome)
    }
}

fn line_to_moves(line: &str) -> (Move, Move) {
    let opp_move = opp_move( &line[..1]);
    let my_move = my_move(&line[2..]);
    (opp_move, my_move)
}

fn line_to_move_and_goal(line: &str) -> (Move, Outcome) {
    let opp_move = opp_move(&line[..1]);
    let my_goal = my_goal(&line[2..]);
    (opp_move, my_goal)
}

fn opp_move(move_str: &str) -> Move {
    match move_str {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Value {} is not a valid opponent move", move_str)
    }
}

fn my_move(move_str: &str) -> Move {
    match move_str {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Value {} is not a valid my move", move_str)
    }
}

fn my_goal(goal_str: &str) -> Outcome {
    match goal_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Value {} is not a valid goal", goal_str)
    }
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}
