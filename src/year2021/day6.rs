use std::fs;

pub(crate) fn run_day_6(should_do_hard_mode: bool) {
    let fishies: Vec<i32> = fs::read_to_string("./Inputs/2021/Day6Input.txt")
        .unwrap()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut fish_buckets: [i128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for i in 0..fishies.len() {
        let i_usize: usize = i.try_into().unwrap();
        let fishie_value: usize = fishies[i_usize].try_into().unwrap();
        fish_buckets[fishie_value] += 1;
    }

    let day_count = if !should_do_hard_mode { 80 } else { 256 };

    for _ in 0..day_count {
        fish_buckets = [
            fish_buckets[1],
            fish_buckets[2],
            fish_buckets[3],
            fish_buckets[4],
            fish_buckets[5],
            fish_buckets[6],
            fish_buckets[7] + fish_buckets[0],
            fish_buckets[8],
            fish_buckets[0],
        ];
    }

    let result: i128 = fish_buckets.iter().sum();

    let problem_id = if !should_do_hard_mode { "6-1" } else { "6-2" };
    println!("Day {}: {}", problem_id, result);
}
