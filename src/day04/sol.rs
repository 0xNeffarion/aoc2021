use std::io::stdin;

const BOARD_SIZE: usize = 5;

#[derive(Clone)]
struct Board {
    squares: Vec<Vec<Square>>,
}

#[derive(Clone)]
struct Square {
    value: u8,
    marked: bool,
}

pub fn solve() {
    let inputs = get_input();
    let order = inputs.0;
    let boards = inputs.1;

    println!("Day 4");
    part_one(&order, &boards);
    part_two(&order, &boards);
}

fn check_win(board: &Board) -> bool {
    let mut counter;

    // Check vertical values
    for r in 0..BOARD_SIZE {
        counter = 0;
        for c in 0..BOARD_SIZE {
            if board.squares[r][c].marked {
                counter += 1;
            }
        }

        if counter == BOARD_SIZE {
            return true;
        }
    }

    // Check horizontal values
    for c in 0..BOARD_SIZE {
        counter = 0;
        for r in 0..BOARD_SIZE {
            if board.squares[r][c].marked {
                counter += 1;
            }
        }

        if counter == BOARD_SIZE {
            return true;
        }
    }

    false
}

fn mark_value(value: u8, board: &mut Board) {
    for r in 0..BOARD_SIZE {
        for c in 0..BOARD_SIZE {
            if board.squares[r][c].value == value {
                board.squares[r][c].marked = true;
                return;
            }
        }
    }
}

fn find_winner(order: &[u8], bds: &[Board], start: usize) -> (Board, usize, u32) {
    let mut boards = bds.to_vec();
    let mut winner = 0;
    let mut last_called = 0;

    'outer: for i in start..order.len() {
        for b in 0..boards.len() {
            let mut board = boards[b].clone();

            mark_value(order[i], &mut board);

            if check_win(&board) {
                winner = b;
                last_called = order[i] as u32;
                break 'outer;
            }

            boards[b] = board;
        }
    }

    (boards[winner].clone(), winner, last_called)
}

fn part_one(order: &[u8], boards: &[Board]) {
    let result = find_winner(order, boards, 0);
    let winner = result.0;
    let last_called = result.2;

    let sum: u32 = winner
        .squares
        .iter()
        .flatten()
        .filter(|x| !x.marked)
        .map(|x| x.value as u32)
        .sum();
    println!("Part 1 - Answer: {}", sum * last_called);
}

fn part_two(order: &[u8], boards: &[Board]) {
    let mut temp_boards = boards.to_vec();
    let mut result = (Board { squares: vec![] }, 0, 0);
    while !temp_boards.is_empty() {
        result = find_winner(order, &temp_boards, 0);
        temp_boards.swap_remove(result.1);
    }

    let sum: u32 = result
        .0
        .squares
        .iter()
        .flatten()
        .filter(|x| !x.marked)
        .map(|x| x.value as u32)
        .sum();
    println!("Part 2 - Answer: {}", sum * result.2);
}

// ------ Parse input ------

fn get_input() -> (Vec<u8>, Vec<Board>) {
    let mut order: Vec<u8> = vec![];

    let mut buff = String::new();
    if let Ok(x) = stdin().read_line(&mut buff) {
        if x > 0 {
            order = buff
                .trim()
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
        }
    };

    (order, read_boards())
}

fn read_boards() -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = Board { squares: vec![] };
    let mut line_break = 0;

    loop {
        let mut buff = String::new();
        match stdin().read_line(&mut buff) {
            Ok(x) => {
                if x == 0 {
                    break;
                }

                if buff.trim().is_empty() {
                    if line_break > 0 {
                        break;
                    }

                    line_break += 1;
                    if !board.squares.is_empty() {
                        boards.push(board);
                    }

                    board = Board { squares: vec![] };
                    continue;
                }

                line_break = 0;
                let values: Vec<Square> = buff
                    .trim()
                    .replace("  ", " ")
                    .split(' ')
                    .map(|x| x.trim().parse::<u8>().unwrap())
                    .map(|v| Square {
                        value: v,
                        marked: false,
                    })
                    .collect::<Vec<Square>>();
                board.squares.push(values);
            }
            Err(_) => {
                break;
            }
        };
    }

    boards
}
