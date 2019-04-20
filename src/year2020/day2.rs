use regex::Regex;
use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let input = fs::read_to_string("./Inputs/2020/Day2Input.txt").unwrap();
    let input_slice: &str = &input[..];

    let re = Regex::new(r"(\d+)-(\d+) (.): ([^\n]+)").unwrap();

    let result = re
        .captures_iter(input_slice)
        .map(|cap| Password {
            min_count: cap[1].parse::<usize>().unwrap(),
            max_count: cap[2].parse::<usize>().unwrap(),
            required_letter: cap[3].chars().next().unwrap(),
            saved_password: cap[4].to_string(),
        })
        .filter(|p| {
            if !should_do_hard_mode {
                let match_count = p.saved_password.matches(p.required_letter).count();
                match_count >= p.min_count && match_count <= p.max_count
            } else {
                let first_letter = p.saved_password.chars().nth(p.min_count - 1).unwrap();
                let second_letter = p.saved_password.chars().nth(p.max_count - 1).unwrap();
                first_letter != second_letter
                    && (first_letter == p.required_letter || second_letter == p.required_letter)
            }
        })
        .count();

    let problem_id = if !should_do_hard_mode { "2-1" } else { "2-2" };
    println!("Day {}: {}", problem_id, result);
}

struct Password {
    min_count: usize,
    max_count: usize,
    required_letter: char,
    saved_password: String,
}
