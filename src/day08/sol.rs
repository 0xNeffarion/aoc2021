use crate::common;
use std::collections::HashMap;
use std::str::FromStr;

struct Signal {
    pattern: Vec<String>,
    output: Vec<String>,
}

const DEFAULT_SCORES: [usize; 10] = [42, 17, 34, 39, 30, 37, 41, 25, 49, 45];

fn score_map(signal: &Signal) -> HashMap<char, usize> {
    let mut scores: HashMap<char, usize> = HashMap::new();
    let phrase: String = signal.pattern.join("");
    scores.insert('a', phrase.chars().filter(|x| *x == 'a').count());
    scores.insert('b', phrase.chars().filter(|x| *x == 'b').count());
    scores.insert('c', phrase.chars().filter(|x| *x == 'c').count());
    scores.insert('d', phrase.chars().filter(|x| *x == 'd').count());
    scores.insert('e', phrase.chars().filter(|x| *x == 'e').count());
    scores.insert('f', phrase.chars().filter(|x| *x == 'f').count());
    scores.insert('g', phrase.chars().filter(|x| *x == 'g').count());

    scores
}

pub fn solve() {
    let signals: Vec<Signal> = common::parse_input();

    println!("Day 8");
    part_one(&signals);
    part_two(&signals);
}

fn part_one(signals: &[Signal]) {
    let seg_map: [usize; 4] = [2, 4, 3, 7];
    let sum: usize = signals
        .iter()
        .map(|s| {
            s.output
                .iter()
                .map(|i| i.len())
                .filter(|x| seg_map.contains(x))
                .count()
        })
        .sum();
    println!("Part 1 - Answer: {}", sum);
}

fn part_two(signals: &[Signal]) {
    let mut result: u64 = 0;
    for s in signals {
        let score_map = score_map(s);
        let mut output = String::new();
        for o in &s.output {
            let sum: usize = o.chars().map(|c| score_map.get(&c).unwrap()).sum();
            let digit = DEFAULT_SCORES.iter().position(|x| *x == sum).unwrap();
            output.push_str(&digit.to_string());
        }

        let value = output.parse::<u64>().unwrap();
        result += value;
    }

    println!("Part 2 - Answer: {}", result);
}

// ----------- Input ----------

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(());
        }

        let exp = s.trim().split('|').collect::<Vec<&str>>();
        if exp.len() < 2 {
            return Err(());
        }

        let patterns = exp[0].trim().to_string();
        let output = exp[1].trim().to_string();

        let mut result_pattern: Vec<String> = vec![];
        let mut result_output: Vec<String> = vec![];
        let pattern_vec = patterns.split(' ').collect::<Vec<&str>>();
        if pattern_vec.len() < 9 {
            return Err(());
        }

        for p in pattern_vec {
            result_pattern.push(p.trim().to_string());
        }

        let output_vec = output.split(' ').collect::<Vec<&str>>();
        if output_vec.len() < 4 {
            return Err(());
        }

        for o in output_vec {
            result_output.push(o.trim().to_string());
        }

        Ok(Signal {
            pattern: result_pattern,
            output: result_output,
        })
    }
}
