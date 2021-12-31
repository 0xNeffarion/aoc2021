use crate::common;

pub fn solve() {
    let state: Vec<u128> = input();

    println!("Day 6");
    part_one(&mut state.clone());
    part_two(&mut state.clone());
}

fn breed_fish(state: &mut Vec<u128>, days: usize) {
    let mut temp = 0;
    (0..days).for_each(|_| {
        temp = state[0];
        state.rotate_left(1);
        state[6] += temp;
        state[8] = temp;
    })
}

fn part_one(state: &mut Vec<u128>) {
    breed_fish(state, 80);

    println!("Part 1 - Answer: {}", state.iter().sum::<u128>());
}

fn part_two(state: &mut Vec<u128>) {
    breed_fish(state, 256);

    println!("Part 1 - Answer: {}", state.iter().sum::<u128>());
}

fn input() -> Vec<u128> {
    let mut state: Vec<u128> = vec![0; 9];
    let line = common::raw_input();
    let split: Vec<&str> = line[0].trim().split(',').collect();

    split
        .iter()
        .map(|x| x.trim().parse::<u128>().unwrap())
        .for_each(|x| {
            state[x as usize] += 1;
        });

    state
}
