use crate::common;

type Heightmap = Vec<Vec<u8>>;

pub fn solve() {
    let map = input();
    println!("Day 9");

    part_one(&map);
    part_two(&map);
}

fn part_two(map: &Heightmap) {
    let mut basins = find_basins(map);
    basins.sort_unstable();
    let top: usize = basins.iter().rev().take(3).product();
    println!("Part 2 - Answer: {}", top);
}

fn find_basins(map: &Heightmap) -> Vec<usize> {
    let low_points = low_points(map);
    let mut sizes = vec![];

    for x in low_points {
        let mut bss: Vec<(usize, usize)> = vec![];
        recursive_basin(map, x.0, x.1, &mut bss);
        sizes.push(bss.len())
    }

    sizes
}

fn recursive_basin(map: &Heightmap, row: usize, col: usize, result: &mut Vec<(usize, usize)>) {
    if map[row][col] == 9 {
        return;
    }

    if !result.contains(&(row, col)) {
        result.push((row, col));
    }

    let indices = value_low_points_indices(map, row, col);
    for x in indices {
        recursive_basin(map, x.0, x.1, result);
    }
}

fn value_low_points_indices(
    map: &Heightmap,
    row_index: usize,
    col_index: usize,
) -> Vec<(usize, usize)> {
    let mut points = vec![];
    let num = map[row_index][col_index];
    if num == 9 {
        return points;
    }

    let up = map[row_index - 1][col_index];
    if up != 9 && num < up {
        points.push((row_index - 1, col_index))
    }

    let down = map[row_index + 1][col_index];
    if down != 9 && num < down {
        points.push((row_index + 1, col_index))
    }

    let left = map[row_index][col_index - 1];
    if left != 9 && num < left {
        points.push((row_index, col_index - 1))
    }

    let right = map[row_index][col_index + 1];
    if right != 9 && num < right {
        points.push((row_index, col_index + 1))
    }

    points
}

fn part_one(map: &Heightmap) {
    let points = low_points(map)
        .iter()
        .map(|x| map[x.0][x.1] as usize)
        .collect::<Vec<usize>>();
    let sum: usize = points.iter().map(|x| (*x) + 1).sum();
    println!("Part 1 - Answer: {}", sum)
}

fn low_points(map: &Heightmap) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let rows = map.len();
    let cols = map[0].len();

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if is_low_point(map, r, c) {
                result.push((r, c));
            }
        }
    }

    result
}

fn is_low_point(map: &Heightmap, row_index: usize, col_index: usize) -> bool {
    let num = map[row_index][col_index];
    let up = map[row_index - 1][col_index];
    let down = map[row_index + 1][col_index];
    let left = map[row_index][col_index - 1];
    let right = map[row_index][col_index + 1];

    [up, down, left, right].into_iter().all(|x| num < x)
}

// -------- Read input -------

fn input() -> Heightmap {
    let lines = common::raw_input();
    let cols = lines[0].trim().len();
    let mut result: Heightmap = Vec::with_capacity(lines.len() + 2);
    result.push(vec![9; cols + 2]);
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let nums = line
            .trim()
            .chars()
            .filter(|x| x.is_digit(10))
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();

        let mut n = vec![9];
        for x in nums {
            n.push(x)
        }
        n.push(9);

        result.push(n);
    }

    result.push(vec![9; cols + 2]);
    result
}
