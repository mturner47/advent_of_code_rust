use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let map_lines: Vec<Vec<char>> = fs::read_to_string("./Inputs/2020/Day3Input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    let movements = if !should_do_hard_mode {
        vec![(1, 3)]
    } else {
        vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    };

    let result: i64 = movements
        .iter()
        .map(|m| {
            let mut num_trees = 0;
            let (down_move, right_move) = m;
            let mut current_square = 0;
            let line_length = map_lines[0].len();
            for line in map_lines.iter().step_by(*down_move) {
                if line[current_square] == '#' {
                    num_trees += 1;
                }
                current_square = (current_square + right_move) % line_length;
            }
            num_trees
        })
        .product();

    let problem_id = if !should_do_hard_mode { "3-1" } else { "3-2" };
    println!("Day {}: {}", problem_id, result);
}
