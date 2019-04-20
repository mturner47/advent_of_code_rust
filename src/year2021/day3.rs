use std::fs;

pub(crate) fn run_day_3(should_do_hard_mode: bool) {
    let filename = "./Inputs/2021/Day3Input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let line_separator = '\n';

    let bit_strings: Vec<&str> = contents.split(line_separator).collect();

    let result = if !should_do_hard_mode {
        get_value_for_easy_mode(&bit_strings)
    } else {
        get_value_for_hard_mode(&bit_strings)
    };

    let problem_id = if !should_do_hard_mode { "3-1" } else { "3-2" };
    println!("Day {}: {}", problem_id, result);
}

fn get_value_for_easy_mode(bit_strings: &Vec<&str>) -> i32 {
    let mut total_vec: Vec<i32> = vec![0i32; bit_strings[0].len().try_into().unwrap()];
    let mut gamma_vec: Vec<i32> = vec![0i32; bit_strings[0].len().try_into().unwrap()];
    let mut epsilon_vec: Vec<i32> = vec![0i32; bit_strings[0].len().try_into().unwrap()];
    for bs in bit_strings {
        for (i, c) in bs.chars().enumerate() {
            let usize_i: usize = i.try_into().unwrap();
            total_vec[usize_i] += if c == '0' { -1 } else { 1 };
        }
    }

    for i in 0..total_vec.len() {
        let usize_i: usize = i.try_into().unwrap();
        if total_vec[usize_i] >= 0 {
            gamma_vec[usize_i] = 1;
        } else {
            epsilon_vec[usize_i] = 1;
        }
    }

    let gamma = convert_bits_to_int(&gamma_vec);
    let epsilon = convert_bits_to_int(&epsilon_vec);

    return gamma * epsilon;
}

fn get_value_for_hard_mode(bit_strings: &Vec<&str>) -> i32 {
    let num_strings = bit_strings.len();
    let string_size = bit_strings[0].len();
    let mut bit_ints: Vec<Vec<i32>> = vec![vec![0i32; string_size]; num_strings];

    for j in 0..bit_strings.len() {
        let usize_j: usize = j.try_into().unwrap();
        for (i, c) in bit_strings[usize_j].chars().enumerate() {
            if c == '1' {
                let usize_i: usize = i.try_into().unwrap();
                bit_ints[usize_j][usize_i] = 1;
            }
        }
    }

    let oxygen = filter_values(&bit_ints, 0, true);
    let co2 = filter_values(&bit_ints, 0, false);
    return oxygen * co2;
}

fn filter_values(bit_ints: &Vec<Vec<i32>>, index: usize, use_highest: bool) -> i32 {
    let mut sum: i32 = 0;
    for bi in bit_ints {
        sum += if bi[index] == 0 { -1 } else { 1 };
    }

    let mut val_to_keep = 0;
    if (sum == 0 && use_highest) || (use_highest && sum > 0) || (!use_highest && sum < 0) {
        val_to_keep = 1;
    }

    let new_ints: Vec<Vec<i32>> = bit_ints
        .iter()
        .filter(|bi| bi[index] == val_to_keep)
        .cloned()
        .collect();

    if new_ints.len() == 1 {
        return convert_bits_to_int(&new_ints[0]);
    }

    return filter_values(&new_ints, index + 1, use_highest);
}

fn convert_bits_to_int(bits: &Vec<i32>) -> i32 {
    let mut multiplier = 1;
    let mut sum = 0;
    for i in (0..bits.len()).rev() {
        sum = sum + bits[i] * multiplier;
        multiplier *= 2;
    }
    return sum;
}
