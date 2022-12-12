use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", top_crates(&input, CraneVersion::CrateMover9000));
    println!("Part 2: {}", top_crates(&input, CraneVersion::CrateMover9001))
}

struct Move {
    number: usize,
    from: usize,
    to: usize
}

struct Crates {
    crates: Vec<Vec<char>>
}

enum CraneVersion {
    CrateMover9000,
    CrateMover9001
}

impl Crates {

    fn do_move_9000(&mut self, a_move: Move) {
        for _ in 0..a_move.number {
            let from = &mut self.crates[a_move.from];
            let a_crate = from.pop().expect("out of crates!");
            let to = &mut self.crates[a_move.to];
            to.push(a_crate);
        }
    }

    fn do_move_9001(&mut self, a_move: Move) {
        let from = &mut self.crates[a_move.from];
        let mut temp = Vec::new();
        for _ in 0..a_move.number {
            temp.push(from.pop().expect("out of crates!"));
        }
        let to = &mut self.crates[a_move.to];
        temp.iter().rev().for_each(|a_crate| to.push(*a_crate));
    }

    fn top_crates(&self) -> String {
        self.crates.iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect::<String>()
    }
}

fn top_crates(input: &str, crane_version: CraneVersion) -> String {
    let parsed = parse_input(input);
    let mut crates = parsed.0;
    let moves = parsed.1;
    for a_move in moves {
        match crane_version {
            CraneVersion::CrateMover9000 => crates.do_move_9000(a_move),
            CraneVersion::CrateMover9001 => crates.do_move_9001(a_move)
        };
    }
    crates.top_crates()
}

fn parse_input(input: &str) -> (Crates, Vec<Move>) {
    let mut crate_lines: Vec<&str> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut stack_count: usize = 0;
    for line in input.lines() {
        if line.contains('[') {
            crate_lines.push(line);
        } else if line.contains("   ") {
            stack_count = count_stacks(line);
        } else if line.starts_with("move") {
            moves.push(parse_move(line));
        }
    };
    let crates = init_crates(stack_count, crate_lines);
    (crates, moves)
}

fn count_stacks(line: &str) -> usize {
    line.split("   ").last().expect("invalid stacks line")
        .trim().parse().expect("stacks line did not have a number")
}

fn init_crates(stack_count: usize, crate_lines: Vec<&str>) -> Crates {
    let mut crates = vec![Vec::new(); stack_count];
    for line in crate_lines.iter().rev() {
        let chars = line.chars();
        for (i, ch) in chars.enumerate() {
            if ch.is_uppercase() {
                let which_crate = (i - 1) / 4;
                let stack: &mut Vec<char> = &mut crates[which_crate];
                stack.push(ch);
            }
        }
    }
    Crates {
        crates
    }
}

fn parse_move(line: &str) -> Move {
    let from_index = line.find(" from ").expect("missing 'from' in line");
    let to_index = line.find(" to ").expect("missing 'to' in line");
    let number: usize = (&line[5..from_index]).trim().parse()
        .expect("unparseable 'number' value");
    let from: usize = (&line[from_index+6..to_index]).trim().parse()
        .expect("unparseable 'from' value");
    let to: usize = (&line[to_index+4..]).trim().parse()
        .expect("unparseable 'to' value");
    Move {
        number,
        from: from - 1,
        to: to - 1
    }
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}