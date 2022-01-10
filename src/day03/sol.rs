use crate::common::raw_input;

pub fn solve() {
    let diagnostic: Vec<String> = raw_input();
    let converted = convert_input(&diagnostic);

    println!("Day 3");
    part_one(&converted);
    part_two(&converted);
}

fn convert_input(diagnostic: &[String]) -> Vec<Vec<char>> {
    diagnostic
        .iter()
        .map(|x| x.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn common_bit(input: &[Vec<char>], column: usize) -> char {
    let mut zero = 0;
    let mut one = 0;

    for i in input {
        if i[column] == '0' {
            zero += 1;
        } else if i[column] == '1' {
            one += 1;
        }
    }

    if zero > one {
        return '0';
    }

    '1'
}

fn bits_of(input: &[Vec<char>], column: usize, value: char) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    for i in input {
        if i[column] == value {
            result.push(i.to_vec());
        }
    }

    result
}

fn part_one(diagnostic: &[Vec<char>]) {
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();
    let columns_len = diagnostic[0].len();

    for i in 0..columns_len {
        let most_common = common_bit(&diagnostic.to_vec(), i);
        let least_common = if most_common == '0' { '1' } else { '0' };

        gamma_str.push(most_common);
        epsilon_str.push(least_common);
    }

    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();
    let power_consumption = gamma * epsilon;
    println!("Part 1 - Answer: {}", power_consumption);
}

fn part_two(diagnostic: &[Vec<char>]) {
    let mut oxy_vals = diagnostic.to_vec();
    let mut co2_vals = diagnostic.to_vec();

    let columns = diagnostic[0].len();

    for i in 0..columns {
        if co2_vals.len() == 1 && oxy_vals.len() == 1 {
            break;
        }

        if oxy_vals.len() != 1 {
            let common = common_bit(&oxy_vals, i);
            oxy_vals = bits_of(&oxy_vals, i, common);
        }

        if co2_vals.len() != 1 {
            let least = if common_bit(&co2_vals, i) == '0' {
                '1'
            } else {
                '0'
            };
            co2_vals = bits_of(&co2_vals, i, least);
        }
    }

    let oxy_str = oxy_vals[0].iter().cloned().collect::<String>();
    let co2_str = co2_vals[0].iter().cloned().collect::<String>();

    let oxygen = isize::from_str_radix(&oxy_str, 2).unwrap();
    let co2 = isize::from_str_radix(&co2_str, 2).unwrap();
    let life_support = oxygen * co2;
    println!("Part 2 - Answer: {}", life_support);
}
