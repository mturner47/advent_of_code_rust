use std::fs;

pub(crate) fn run_day_1(should_use_sliding_window: bool) {
    let filename = "./Inputs/2021/Day1Input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let line_separator = '\n';

    let lines: Vec<i32> = contents
        .split(line_separator)
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let mut count = 0;

    let window_size = if should_use_sliding_window { 3 } else { 1 };

    for i in window_size..lines.len() {
        if lines[i - window_size] < lines[i] {
            count += 1;
        }
    }

    let problem_id = if !should_use_sliding_window {
        "1-1"
    } else {
        "1-2"
    };
    println!("Day {}: {}", problem_id, count);
}
