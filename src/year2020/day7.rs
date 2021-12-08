use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let raw_data = fs::read_to_string("./Inputs/2020/Day7Input.txt").unwrap();

    let rules:Vec<Rule> = raw_data
        .split("\n")
        .map(|l| convert_to_rule(l))
        .collect();

    let result = if !should_do_hard_mode { get_easy_result(rules) } else { get_hard_results(&rules) };

    let problem_id = if !should_do_hard_mode { "7-1" } else { "7-2" };
    println!("Day {}: {}", problem_id, result);
}

fn get_easy_result(rules: Vec<Rule>) -> usize {
    let bag_color_to_check = &"shiny gold".to_string();
    let mut colors_to_check:Vec<&String> = Vec::new();
    colors_to_check.push(bag_color_to_check);
    let mut new_colors_to_check: Vec<&String>;
    let mut found_colors: Vec<&String> = Vec::new();

    loop {
        new_colors_to_check = Vec::new();
        for color_to_check in colors_to_check {
            let colors:Vec<&String> = rules.iter().filter(|r| {
                let a = r.held_bags.iter().any(|hb| {
                    hb.bag_color == *color_to_check
                });
                let c = a;
                c
            }).map(|hb| &hb.holder_bag_color)
            .collect();

            let colors_pointer = &colors;
            found_colors.extend(colors_pointer.into_iter());
            new_colors_to_check.extend(colors_pointer.into_iter());
        }
        if new_colors_to_check.len() == 0 {
            break;
        } else {
            colors_to_check = new_colors_to_check;
        }
    }

    found_colors.sort();
    found_colors.dedup();
    found_colors.len()
}

fn get_hard_results(rules: &Vec<Rule>) -> usize {
    let bag_color_to_check = &"shiny gold".to_string();
    let rule_to_check = rules.iter().filter(|r| r.holder_bag_color == *bag_color_to_check).next().unwrap();

    // subtract 1, as we usually want to include the bag we're checking in our counts, but not for our starting gold bag
    get_held_bags(rule_to_check, &rules) - 1
}

fn get_held_bags(rule_to_check: &Rule, rules: &Vec<Rule>) -> usize {
    if rule_to_check.held_bags.len() == 0 {
        return 1;
    }

    let bag_count:usize = rule_to_check.held_bags.iter().map(|hb| {
        let bag_color = &hb.bag_color;
        let bag_count = hb.bag_count;
        let bag_rule = rules.iter().filter(|r| r.holder_bag_color == *bag_color).next().unwrap();
        let held_bag_count = get_held_bags(bag_rule, rules);
        let num_bags:usize = bag_count*held_bag_count;
        num_bags
    }).sum();
    1usize + bag_count
}

struct HeldBag {
    bag_color: String,
    bag_count: usize,
}

struct Rule {
    holder_bag_color: String,
    held_bags: Vec<HeldBag>,
}

fn convert_to_rule(rule_string: &str) -> Rule {
    let rule_parts:Vec<&str> = rule_string.trim_end_matches(".").split(" contain ").collect();
    let holder_bag_color = rule_parts[0].replace(" bags", "").replace(" bag", "");
    let held_bags: Vec<HeldBag> = match rule_parts[1] {
        "no other bags" => Vec::new(),
        _ => {
            rule_parts[1].split(", ")
            .map(|hb| hb.replace(" bags", "").replace(" bag", ""))
            .map(|hb| {
                let first_space_index = hb.find(" ").unwrap();
                let (bag_count, bag_color) = hb.split_at(first_space_index);
                let bag_count_usize = match bag_count.parse::<usize>() {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        0
                    },
                };
                HeldBag {
                    bag_color: bag_color.trim().to_string(),
                    bag_count: bag_count_usize,
                }
            })
            .collect()
        }
    };

    Rule {
        holder_bag_color,
        held_bags,
    }
}
