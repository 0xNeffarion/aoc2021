use crate::common::parse_input;

pub fn solve() {
    let measurements: Vec<i32> = parse_input();

    println!("Day 1");
    part_one(&measurements);
    part_two(&measurements);
}

fn part_one(measurements: &[i32]) {
    let mut counter = 0;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            counter += 1;
        }
    }

    println!("Part 1 - Answer: {}", counter);
}

fn part_two(measurements: &[i32]) {
    let mut counter = 0;
    let mut sums = vec![];
    for i in (0..measurements.len()).step_by(4) {
        for x in i..(i + 4) {
            let window = sum_window(measurements, x);
            sums.push(window);
        }
    }

    for i in 1..sums.len() {
        if sums[i] > sums[i - 1] {
            counter += 1;
        }
    }

    println!("Part 2 - Answer: {}", counter);
}

fn sum_window(measurements: &[i32], start_index: usize) -> i32 {
    let target;
    if (start_index + 3) >= measurements.len() {
        target = measurements.len();
    } else {
        target = start_index + 3;
    }

    (start_index..target).map(|n| measurements[n]).sum::<i32>()
}
