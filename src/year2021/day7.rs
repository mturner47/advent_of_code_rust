use std::fs;

pub(crate) fn run_day_7(should_do_hard_mode: bool) {
    let crab_positions: Vec<usize> = fs::read_to_string("./Inputs/2021/Day7Input.txt")
        .unwrap()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    let max_value = crab_positions.iter().max().unwrap();

    let mut crab_buckets: Vec<usize> = vec![0; *max_value + 1];

    for cp in &crab_positions {
        crab_buckets[*cp] += 1;
    }

    let mut result = 1000000000;

    for i in 0..=*max_value {
        let mut sum = 0;
        for j in 0..=*max_value {
            let diff = if i < j { j - i } else { i - j };
            let fuel_cost = if !should_do_hard_mode {
                diff
            } else {
                diff * (diff + 1) / 2
            };
            sum += fuel_cost * crab_buckets[j];
        }
        if sum < result {
            result = sum;
        }
    }

    let problem_id = if !should_do_hard_mode { "6-1" } else { "6-2" };
    println!("Day {}: {}", problem_id, result);
}
