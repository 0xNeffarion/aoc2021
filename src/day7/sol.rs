use crate::common;

pub fn solve() {
    let crabs = input();
    println!("Day 7");

    part_one(crabs.clone());
    part_two(crabs);
}

fn median(crabs: &[i64]) -> i64 {
    let mut pos = crabs.to_owned();
    let len = pos.len();
    pos.sort_unstable();

    let even = (len % 2) == 0;

    if even {
        let ind_left = (len / 2) - 1;
        let ind_right = len / 2;
        return ((((pos[ind_left]) as f64) + (pos[ind_right] as f64)) / 2.0).round() as i64;
    }

    pos[len / 2]
}

fn mean(crabs: &[i64]) -> f64 {
    (crabs.iter().sum::<i64>() as f64) / (crabs.len() as f64)
}

fn part_one(crabs: Vec<i64>) {
    let median = median(&crabs);
    let fuel = crabs.iter().fold(0, |mut acc, c| {
        acc += (c - median).abs();
        acc
    });
    println!("Part 1 - Answer: {}", fuel);
}

const FUEL_COST: fn(f64) -> f64 = |x| x * (x + 1.0) / 2.0;

fn calculate_fuel(crabs: &[i64], target: f64) -> f64 {
    let mut cost = 0.0;
    for i in crabs {
        cost += FUEL_COST(((*i as f64) - target).abs() as f64);
    }

    cost
}

fn part_two(crabs: Vec<i64>) {
    let mean: f64 = mean(&crabs);
    let fuel_cost = calculate_fuel(&crabs, mean.floor()).min(calculate_fuel(&crabs, mean.ceil()));

    println!("Part 2 - Answer: {}", fuel_cost);
}

fn input() -> Vec<i64> {
    common::raw_input()[0]
        .trim()
        .split(',')
        .into_iter()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
