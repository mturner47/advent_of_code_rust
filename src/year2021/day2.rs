use std::fs;

pub(crate) fn run_day_2(should_use_complex_movement: bool) {
    let filename = "./Inputs/2021/Day2Input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let line_separator = '\n';

    let movements: Vec<Movement> = contents
        .split(line_separator)
        .map(|l| convert_string_to_movement(l))
        .collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for m in movements {
        if should_use_complex_movement {
            match m.direction {
                Direction::Forward => {
                    horizontal_position += m.distance;
                    depth += m.distance * aim;
                }
                Direction::Down => aim += m.distance,
                Direction::Up => aim -= m.distance,
            };
        } else {
            match m.direction {
                Direction::Forward => horizontal_position += m.distance,
                Direction::Down => depth += m.distance,
                Direction::Up => depth -= m.distance,
            };
        }
    }

    let result = horizontal_position * depth;

    let problem_id = if !should_use_complex_movement {
        "2-1"
    } else {
        "2-2"
    };
    println!("Day {}: {}", problem_id, result);
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Movement {
    direction: Direction,
    distance: i32,
}

fn convert_string_to_movement(movement_string: &str) -> Movement {
    let movement_parts: Vec<&str> = movement_string.split(' ').collect();
    let direction_string = movement_parts[0];
    let distance_string = movement_parts[1];
    let direction = match direction_string {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => Direction::Forward,
    };

    let distance = distance_string.parse::<i32>().unwrap();

    Movement {
        direction,
        distance,
    }
}
