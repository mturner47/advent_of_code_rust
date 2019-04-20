use std::fs;

pub(crate) fn run_day_4(should_do_hard_mode: bool) {
    let filename = "./Inputs/2021/Day4Input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let line_separator = '\n';

    let lines: Vec<&str> = contents.split(line_separator).collect();

    let bingo_numbers: Vec<i32> = lines[0]
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let mut bingo_boards: Vec<BingoBoard> = lines[1..]
        .chunks(6)
        .map(|c| convert_to_bingo_board(c))
        .collect();

    let mut result = 0;
    for bingo_number in bingo_numbers {
        bingo_boards = update_bingo_boards_with_number(&bingo_boards, bingo_number);
        let (winning_boards, losing_boards): (Vec<BingoBoard>, Vec<BingoBoard>) = bingo_boards
            .into_iter()
            .partition(|bb| is_bingo_board_a_winner(&bb));
        if winning_boards.len() == 1 && (!should_do_hard_mode || losing_boards.len() == 0) {
            let winning_board_sum = sum_bingo_board(&winning_boards[0]);
            result = winning_board_sum * bingo_number;
            break;
        }
        bingo_boards = losing_boards;
    }

    let problem_id = if !should_do_hard_mode { "4-1" } else { "4-2" };
    println!("Day {}: {}", problem_id, result);
}

struct BingoSpace {
    has_been_marked: bool,
    value: i32,
}

struct BingoLine {
    spaces: Vec<BingoSpace>,
}

struct BingoBoard {
    lines: Vec<BingoLine>,
}

fn convert_to_bingo_board(lines: &[&str]) -> BingoBoard {
    BingoBoard {
        lines: lines[1..]
            .iter()
            .map(|l| convert_line_to_bingo_line(l))
            .collect(),
    }
}

fn convert_line_to_bingo_line(line: &str) -> BingoLine {
    BingoLine {
        spaces: line
            .trim()
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .map(|i| BingoSpace {
                has_been_marked: false,
                value: i,
            })
            .collect(),
    }
}

fn update_bingo_boards_with_number(boards: &Vec<BingoBoard>, number: i32) -> Vec<BingoBoard> {
    boards
        .iter()
        .map(|b| update_bingo_board_with_number(b, number))
        .collect()
}

fn update_bingo_board_with_number(board: &BingoBoard, number: i32) -> BingoBoard {
    BingoBoard {
        lines: board
            .lines
            .iter()
            .map(|l| update_bingo_line_with_number(l, number))
            .collect(),
    }
}

fn update_bingo_line_with_number(line: &BingoLine, number: i32) -> BingoLine {
    BingoLine {
        spaces: line
            .spaces
            .iter()
            .map(|s| BingoSpace {
                has_been_marked: s.has_been_marked || s.value == number,
                value: s.value,
            })
            .collect(),
    }
}

fn sum_bingo_board(board: &BingoBoard) -> i32 {
    board.lines.iter().map(|l| sum_bingo_line(l)).sum()
}

fn sum_bingo_line(line: &BingoLine) -> i32 {
    line.spaces
        .iter()
        .filter(|s| !s.has_been_marked)
        .map(|s| s.value)
        .sum()
}

fn is_bingo_board_a_winner(board: &BingoBoard) -> bool {
    for line in &board.lines {
        if line.spaces.iter().all(|s| s.has_been_marked) {
            return true;
        }
    }

    for i in 0..board.lines.len() {
        let i_usize: usize = i.try_into().unwrap();
        if board
            .lines
            .iter()
            .all(|l| l.spaces[i_usize].has_been_marked)
        {
            return true;
        }
    }

    false
}
