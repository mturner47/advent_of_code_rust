use std::fs;

pub(crate) fn run_day(should_do_hard_mode: bool) {
    let raw_data = fs::read_to_string("./Inputs/2021/Day8Input.txt").unwrap();

    let results:Vec<&str> = raw_data
        .split("\n")
        .collect();

    let result = if !should_do_hard_mode { get_easy_result(results) } else { get_hard_results(results) };

    let problem_id = if !should_do_hard_mode { "7-1" } else { "7-2" };
    println!("Day {}: {}", problem_id, result);
}

fn get_easy_result(results: Vec<&str>) -> usize {
    results.iter().map(|l| {
        let parts: Vec<&str> = l.split(" | ").collect();
        parts[1]
    }).map(|r| {
        r.split(" ").filter(|f| f.len() == 2 || f.len() == 3 || f.len() == 4 || f.len() == 7).count()
    }).sum()
}

fn get_hard_results(results: Vec<&str>) -> usize {
    results.iter().map(|r| {
        let line_parts:Vec<&str> = r.split(" | ").collect();
        let signals:Vec<Vec<char>> = line_parts[0].split(" ").map(|s| s.chars().collect()).collect();
        let answers:Vec<Vec<char>> = line_parts[1].split(" ").map(|s| s.chars().collect()).collect();

        // First four signals are pretty easy, as they're the only possibilities
        let signal_1:&Vec<char> = signals.iter().filter(|f| f.len() == 2).next().unwrap();
        let signal_4:&Vec<char> = signals.iter().filter(|f| f.len() == 4).next().unwrap();
        let signal_7:&Vec<char> = signals.iter().filter(|f| f.len() == 3).next().unwrap();
        let signal_8:&Vec<char> = signals.iter().filter(|f| f.len() == 7).next().unwrap();

        // Next up is the 0, which we can get by finding the two letters in 4 that are missing in 1,
        // and then finding the six character signal that is missing one of those two letters
        let waist_and_left_arm:Vec<&char> = signal_4.into_iter().filter(|c| !signal_1.contains(c)).collect();
        let signal_0:&Vec<char> = signals.iter()
            .filter(|f| f.len() == 6 && f.contains(&waist_and_left_arm[0]) != f.contains(&waist_and_left_arm[1]))
            .next().unwrap();

        // The value that wasn't missing in 0 between 0 and 4 is the left arm, which we'll use to find the 5,
        // the only 5-digit number that has that slot filled.
        let left_arm = if signal_0.contains(&waist_and_left_arm[0]) { waist_and_left_arm[0] } else { waist_and_left_arm[1] };

        let signal_5:&Vec<char> = signals.iter().filter(|f| f.len() == 5 && f.contains(&left_arm)).next().unwrap();

        // 3 is obtained by finding the only 5 digit number that contains both the letters in 1
        let signal_3:&Vec<char> = signals.iter().filter(|f| f.len() == 5 && f.contains(&signal_1[0]) && f.contains(&signal_1[1])).next().unwrap();

        // Since we know 5 and 3, the only 5-digit number left is 2
        let signal_2:&Vec<char> = signals.iter().filter(|f| f.len() == 5 && f != &signal_5 && f != &signal_3).next().unwrap();

        // 9 is a little wonky. We get it by finding the 6-digit number that isn't 0 and contains both letters in 1
        let signal_9:&Vec<char> = signals.iter().filter(|f| f.len() == 6 && f != &signal_0 && f.contains(&signal_1[0]) && f.contains(&signal_1[1])).next().unwrap();

        // Our final number is 6, which is the last 6-digit number (and, technically, the last number overall)
        let signal_6:&Vec<char> = signals.iter().filter(|f| f.len() == 6 && f != &signal_0 && f != &signal_9).next().unwrap();

        let mapped_signals = [signal_0, signal_1, signal_2, signal_3, signal_4, signal_5, signal_6, signal_7, signal_8, signal_9];

        let mut sum = 0;
        let mut multiplier = 1000;

        for a in answers {
            let mut val = 0;
            for (i, s) in mapped_signals.iter().enumerate() {
                if are_equal(&a, s) {
                    val = i;
                }
            }
            sum += val*multiplier;
            multiplier /= 10;
        }
        sum
    }).sum()
}

fn are_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for c in a {
        if !b.contains(c) {
            return false;
        }
    }
    true
}
