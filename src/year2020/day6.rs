use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let raw_data = fs::read_to_string("./Inputs/2020/Day6Input.txt").unwrap();

    let result = raw_data
        .split("\n\n")
        .map(|lg| if !should_do_hard_mode {get_easy_group_sum(lg)} else {get_hard_group_sum(lg)})
        .sum::<usize>();

    let problem_id = if !should_do_hard_mode { "6-1" } else { "6-2" };
    println!("Day {}: {}", problem_id, result);
}

fn get_easy_group_sum(lg: &str) -> usize {
    let mut chars:Vec<char> = lg.split("\n").collect::<Vec<&str>>().join("").chars().collect();
    chars.sort();
    chars.dedup();
    chars.len()
}

fn get_hard_group_sum(lg: &str) -> usize {
    let mut lines:Vec<&str> = lg.split("\n").collect();
    lines.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    let shortest_line_chars:Vec<char> = lines[0].chars().collect::<Vec<char>>();
    shortest_line_chars.iter().filter(|c| lines.iter().all(|l| l.contains(**c))).count()
}