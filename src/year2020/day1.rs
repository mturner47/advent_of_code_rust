use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let invoice_entries: Vec<i32> = fs::read_to_string("./Inputs/2020/Day1Input.txt")
        .unwrap()
        .split('\n')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut result = 0;
    for i in &invoice_entries {
        for j in &invoice_entries {
            if !should_do_hard_mode {
                if i + j == 2020 {
                    result = i * j;
                }
            } else {
                for k in &invoice_entries {
                    if i + j + k == 2020 {
                        result = i * j * k;
                    }
                }
            }
        }
    }

    let problem_id = if !should_do_hard_mode { "1-1" } else { "1-2" };
    println!("Day {}: {}", problem_id, result);
}
