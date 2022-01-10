use crate::common;
use phf::phf_map;
use std::collections::VecDeque;

type Syntax = Vec<Vec<char>>;

static CORRUPT_SCORES: phf::Map<char, u64> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
};

static INCOMPLETE_SCORES: phf::Map<char, u64> = phf_map! {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4
};

const OPEN_CHARS: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_CHARS: [char; 4] = [')', ']', '}', '>'];

pub fn solve() {
    let syntax = input();
    println!("Day 10");

    part_one(&syntax);
    part_two(&syntax);
}

fn index_of(chars: [char; 4], c: char) -> i32 {
    for i in 0..chars.len() {
        if chars[i] == c {
            return i as i32;
        }
    }

    -1
}

fn check_corruption_chunk(chunk: &[char]) -> Option<u64> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in chunk {
        if OPEN_CHARS.contains(c) {
            stack.push_back(*c);
        } else {
            for (i, v) in CLOSE_CHARS.iter().enumerate() {
                if *v == *c {
                    match stack.pop_back() {
                        Some(v) => {
                            let index = index_of(OPEN_CHARS, v);
                            if index == -1 || index != (i as i32) {
                                return Some(*CORRUPT_SCORES.get(c).unwrap());
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
    }

    None
}

fn part_one(syntax: &Syntax) {
    let syntax = syntax.to_vec();
    let points: u64 = syntax
        .iter()
        .map(|x| check_corruption_chunk(x))
        .fold(0, |acc, x| acc + x.unwrap_or(0));

    println!("Part 1 - Answer: {}", points);
}

fn check_incomplete_chunk(chunk: &[char]) -> Vec<char> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in chunk {
        if OPEN_CHARS.contains(c) {
            stack.push_back(*c);
        } else {
            for v in CLOSE_CHARS {
                if v == *c {
                    stack.pop_back();
                    break;
                }
            }
        }
    }

    let mut result = Vec::from_iter(stack.into_iter());
    result.reverse();
    result
}

fn part_two(syntax: &Syntax) {
    let syntax = syntax.to_vec();
    let clean = syntax
        .into_iter()
        .filter(|x| check_corruption_chunk(x).is_none())
        .collect::<Vec<Vec<char>>>();
    let incomplete = clean
        .iter()
        .map(|x| check_incomplete_chunk(x))
        .collect::<Vec<Vec<char>>>();
    let mut scores = incomplete
        .iter()
        .map(|x| {
            x.iter()
                .fold(0, |acc, v| ((acc * 5) + INCOMPLETE_SCORES[v]))
        })
        .collect::<Vec<u64>>();

    scores.sort_unstable();
    let middle = scores[scores.len() / 2];

    println!("Part 2 - Answer: {}", middle);
}

fn input() -> Syntax {
    common::raw_input()
        .iter()
        .map(|x| x.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
