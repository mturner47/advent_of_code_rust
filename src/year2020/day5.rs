use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let raw_data = fs::read_to_string("./Inputs/2020/Day5Input.txt").unwrap();

    let seat_values: Vec<i32> = raw_data
        .split("\n")
        .map(|bp| {
            let (row_string, col_string) = bp.split_at(7);
            let row = convert_from_binary(row_string);
            let col = convert_from_binary(col_string);
            row*8 + col
        }).collect();

    let result = match should_do_hard_mode {
        false => *seat_values.iter().max().unwrap(),
        true => seat_values.iter().filter(|sv| !seat_values.contains(&(**sv - 1)) && seat_values.contains(&(*sv - 2))).next().unwrap() - 1
    };
    let problem_id = if !should_do_hard_mode { "5-1" } else { "5-2" };
    println!("Day {}: {}", problem_id, result);
}

fn convert_from_binary(s: &str) -> i32 {
    let mut sum = 0;
    let mut multiplier = 1;
    for c in s.chars().rev() {
        let val = if c == 'F' || c == 'L' {0} else {1};
        sum += val * multiplier;
        multiplier *= 2;
    }
    sum
}
