use crate::common;

const OFFSETS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const SIZE: usize = 10;

pub fn solve() {
    let grid = input();
    println!("Day 11");

    part_one(&grid);
    part_two(&grid);
}

fn increment(grid: &mut [Vec<i64>]) {
    for i in 1..SIZE + 1 {
        for j in 1..SIZE + 1 {
            grid[i][j] += 1;
        }
    }
}

fn normalize(grid: &mut [Vec<i64>]) {
    for i in 1..SIZE + 1 {
        for j in 1..SIZE + 1 {
            if grid[i][j] < 0 {
                grid[i][j] = 0;
            }
        }
    }
}

fn step(grid: &mut [Vec<i64>]) -> u64 {
    let mut result: u64 = 0;

    increment(grid);
    loop {
        let mut flashes = 0;
        for i in 1..SIZE + 1 {
            for j in 1..SIZE + 1 {
                if grid[i][j] > 9 {
                    for (x, y) in OFFSETS {
                        grid[(i as i64 + x) as usize][(j as i64 + y) as usize] += 1;
                    }
                    grid[i][j] = i64::MIN;
                    flashes += 1;
                }
            }
        }

        result += flashes;
        if flashes == 0 {
            break;
        }
    }

    normalize(grid);
    result
}

fn part_one(grid: &[Vec<i64>]) {
    let mut grid = grid.to_vec();
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }

    println!("Part 1 - Answer: {}", sum);
}

fn part_two(grid: &[Vec<i64>]) {
    let mut grid = grid.to_vec();
    let mut count = 1;
    let target = (SIZE * SIZE) as u64;
    loop {
        let flashes = step(&mut grid);
        if flashes == target {
            break;
        }

        count += 1;
    }

    println!("Part 2 - Answer: {}", count);
}

// -------- Read input ----------

fn input() -> Vec<Vec<i64>> {
    let lines = common::raw_input();
    let mut result = vec![vec![0; SIZE + 2]];
    for x in lines {
        let mut vc: Vec<i64> = vec![0];
        let chars = x.trim().chars().collect::<Vec<char>>();
        chars
            .iter()
            .map(|x| x.to_digit(10).unwrap())
            .for_each(|x| vc.push(x as i64));
        vc.push(0);
        result.push(vc);
    }
    result.push(vec![0; SIZE + 2]);
    result
}
