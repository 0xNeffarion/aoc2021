use std::num::ParseIntError;
use std::ops::RangeInclusive;

use crate::common;

#[derive(Clone, Debug)]
struct Field {
    values: Vec<Vec<i32>>,
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

const FIELD_SIZE: usize = 1000;
const MIN_OVERLAP: i32 = 2;

pub fn solve() {
    let paths: Vec<(Point, Point)> = parse_input().unwrap();
    let mut field = Field {
        values: vec![vec![0; FIELD_SIZE]; FIELD_SIZE],
    };

    println!("Day 5");
    part_one(&mut (field.clone()), &paths);
    part_two(&mut field, &paths);
}

fn range(v1: i32, v2: i32) -> RangeInclusive<i32> {
    v1.min(v2)..=v1.max(v2)
}

fn increment(field: &mut Field, x: i32, y: i32) {
    field.values[x as usize][y as usize] += 1;
}

fn apply_path(field: &mut Field, path: &(Point, Point), diagonals: bool) {
    let x1 = path.0.x;
    let x2 = path.1.x;
    let y1 = path.0.y;
    let y2 = path.1.y;

    if x1 == x2 {
        // vertical
        let range = range(y1, y2);
        for y in range {
            increment(field, x1, y);
        }
    } else if y1 == y2 {
        // horizontal
        let range = range(x1, x2);
        for x in range {
            increment(field, x, y1);
        }
    } else if diagonals {
        // diagonals
        recursive_diagonal_primary(field, x1, y1, x2, y2);
        recursive_diagonal_secondary(field, x1, y1, x2, y2);
    }
}

fn recursive_diagonal_primary(field: &mut Field, x1: i32, y1: i32, x2: i32, y2: i32) {
    if x1 == x2 && y1 == y2 {
        increment(field, x1, y1);
        return;
    }

    if x1 > x2 && y1 > y2 {
        recursive_diagonal_primary(field, x1 - 1, y1 - 1, x2, y2);
        increment(field, x1, y1);
    } else if x1 < x2 && y1 < y2 {
        recursive_diagonal_primary(field, x1 + 1, y1 + 1, x2, y2);
        increment(field, x1, y1);
    }
}

fn recursive_diagonal_secondary(field: &mut Field, x1: i32, y1: i32, x2: i32, y2: i32) {
    if x1 == x2 && y1 == y2 {
        increment(field, x1, y1);
        return;
    }

    if y1 > y2 && x1 < x2 {
        recursive_diagonal_secondary(field, x1 + 1, y1 - 1, x2, y2);
        increment(field, x1, y1);
    } else if y1 < y2 && x1 > x2 {
        recursive_diagonal_secondary(field, x1 - 1, y1 + 1, x2, y2);
        increment(field, x1, y1);
    }
}

fn part_one(field: &mut Field, paths: &[(Point, Point)]) {
    for p in paths {
        apply_path(field, p, false);
    }

    let overlaps: usize = (&field.values)
        .iter()
        .flatten()
        .filter(|x| (**x) >= MIN_OVERLAP)
        .count();

    println!("Part 1 - Answer: {}", overlaps);
}

fn part_two(field: &mut Field, paths: &[(Point, Point)]) {
    for p in paths {
        apply_path(field, p, true);
    }

    let overlaps: usize = (&field.values)
        .iter()
        .flatten()
        .filter(|x| (**x) >= MIN_OVERLAP)
        .count();
    println!("Part 2 - Answer: {}", overlaps);
}

// ----- Read input -----

fn parse_input() -> Result<Vec<(Point, Point)>, ParseIntError> {
    let result = common::raw_input();
    let mut output: Vec<(Point, Point)> = vec![];
    for s in result {
        let val = s.trim();
        if !val.is_empty() {
            let arr: Vec<&str> = val.split("->").collect();
            if arr.is_empty() {
                continue;
            }

            let v1_arr: Vec<&str> = arr[0].trim().split(',').collect();
            let v1_x = v1_arr[0].trim().parse::<i32>()?;
            let v1_y = v1_arr[1].trim().parse::<i32>()?;

            let v2_arr: Vec<&str> = arr[1].trim().split(',').collect();
            let v2_x = v2_arr[0].trim().parse::<i32>()?;
            let v2_y = v2_arr[1].trim().parse::<i32>()?;

            let p1 = Point { x: v1_x, y: v1_y };
            let p2 = Point { x: v2_x, y: v2_y };
            output.push((p1, p2));
        }
    }

    Ok(output)
}
