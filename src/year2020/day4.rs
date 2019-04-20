use regex::Regex;
use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let raw_data = fs::read_to_string("./Inputs/2020/Day4Input.txt").unwrap();

    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let hair_color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let passport_id_regex = Regex::new(r"^\d{9}$").unwrap();

    let result = raw_data
        .split("\n\n")
        .filter(|kg| {
            let key_value_pairs: Vec<(&str, &str)> = kg
                .split_whitespace()
                .map(|kvp| {
                    let parts: Vec<&str> = kvp.split(':').collect();
                    let key = parts[0];
                    let value = parts[1];
                    (key, value)
                })
                .collect();

            required_keys
                .iter()
                .map(|rk| {
                    let matching_kvp_option =
                        key_value_pairs.iter().filter(|kvp| kvp.0 == *rk).next();
                    if matching_kvp_option == None {
                        false
                    } else if !should_do_hard_mode {
                        true
                    } else {
                        let matching_kvp = matching_kvp_option.unwrap();
                        let value = matching_kvp.1;
                        match *rk {
                            "byr" => is_in_range(value, 1920, 2002),
                            "iyr" => is_in_range(value, 2010, 2020),
                            "eyr" => is_in_range(value, 2020, 2030),
                            "hgt" => is_valid_height(value),
                            "hcl" => hair_color_regex.is_match(value),
                            "ecl" => valid_eye_colors.contains(&value),
                            "pid" => passport_id_regex.is_match(value),
                            _ => false,
                        }
                    }
                })
                .all(|x| x)
        })
        .count();

    let problem_id = if !should_do_hard_mode { "4-1" } else { "4-2" };
    println!("Day {}: {}", problem_id, result);
}

fn is_in_range(s: &str, min: i32, max: i32) -> bool {
    match s.parse::<i32>() {
        Err(_) => false,
        Ok(value) => value >= min && value <= max,
    }
}

fn is_valid_height(s: &str) -> bool {
    if s.ends_with("cm") {
        is_in_range(&s.replace("cm", ""), 150, 193)
    } else if s.ends_with("in") {
        is_in_range(&s.replace("in", ""), 59, 76)
    } else {
        false
    }
}
