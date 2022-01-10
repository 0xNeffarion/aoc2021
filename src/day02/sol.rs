use std::str::FromStr;

use crate::common::parse_input;

struct Operation {
    command: Move,
    value: u32,
}

enum Move {
    Forward,
    Down,
    Up,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.trim().split(' ').collect::<Vec<&str>>();
        let cmd = if let Ok(v) = Move::from_str(words[0]) {
            v
        } else {
            return Err(());
        };

        let val = if let Ok(v) = words[1].parse::<u32>() {
            v
        } else {
            return Err(());
        };

        Ok(Operation {
            command: cmd,
            value: val,
        })
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "forward" => Ok(Move::Forward),
            "down" => Ok(Move::Down),
            "up" => Ok(Move::Up),
            _ => Err(()),
        }
    }
}

pub fn solve() {
    let commands: Vec<Operation> = parse_input();

    println!("Day 2");
    part_one(&commands);
    part_two(&commands);
}

fn part_one(ops: &[Operation]) {
    let mut depth = 0;
    let mut position = 0;

    for x in ops.iter() {
        match x.command {
            Move::Forward => position += x.value,
            Move::Up => depth -= x.value,
            Move::Down => depth += x.value,
        }
    }

    println!("Part 1 - Answer: {}", depth * position);
}

fn part_two(ops: &[Operation]) {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for x in ops.iter() {
        match x.command {
            Move::Forward => {
                position += x.value;
                depth += aim * x.value;
            }
            Move::Up => aim -= x.value,
            Move::Down => aim += x.value,
        }
    }

    println!("Part 2 - Answer: {}", depth * position);
}
