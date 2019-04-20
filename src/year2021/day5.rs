use std::fs;

pub(crate) fn run_day_5(should_include_diagonals: bool) {
    let filename = "./Inputs/2021/Day5Input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let line_separator = '\n';

    let lines: Vec<Line> = contents
        .split(line_separator)
        .map(|l| convert_line_string_to_line(l))
        .collect();

    let mut vent_map = initialize_vent_map(&lines);

    for line in lines {
        add_line_to_vent_map(&mut vent_map, &line, should_include_diagonals);
    }

    let mut overlap_count = 0;
    for row in vent_map {
        for col in row {
            if col >= 2 {
                overlap_count += 1;
            }
        }
    }

    let problem_id = if !should_include_diagonals {
        "5-1"
    } else {
        "5-2"
    };
    println!("Day {}: {}", problem_id, overlap_count);
}

struct Point {
    x: i32,
    y: i32,
}
struct Line {
    start_point: Point,
    end_point: Point,
}

fn convert_line_string_to_line(line_string: &str) -> Line {
    let point_strings: Vec<&str> = line_string.split(" -> ").collect();
    Line {
        start_point: convert_point_string_to_point(point_strings[0]),
        end_point: convert_point_string_to_point(point_strings[1]),
    }
}

fn convert_point_string_to_point(point_string: &str) -> Point {
    let point_vector: Vec<&str> = point_string.split(',').collect();
    Point {
        x: point_vector[0].parse::<i32>().unwrap(),
        y: point_vector[1].parse::<i32>().unwrap(),
    }
}

fn initialize_vent_map(lines: &[Line]) -> Vec<Vec<i32>> {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        if line.start_point.x > max_x {
            max_x = line.start_point.x;
        }

        if line.end_point.x > max_x {
            max_x = line.end_point.x;
        }

        if line.start_point.y > max_y {
            max_y = line.start_point.y;
        }

        if line.end_point.y > max_y {
            max_y = line.end_point.y;
        }
    }

    return vec![vec![0i32; (max_y + 1).try_into().unwrap()]; (max_x + 1).try_into().unwrap()];
}

fn add_line_to_vent_map(vent_map: &mut Vec<Vec<i32>>, line: &Line, should_include_diagonals: bool) {
    if !should_include_diagonals
        && line.start_point.x != line.end_point.x
        && line.start_point.y != line.end_point.y
    {
        return;
    }

    let start_x: usize = line.start_point.x.try_into().unwrap();
    let start_y: usize = line.start_point.y.try_into().unwrap();
    let end_x: usize = line.end_point.x.try_into().unwrap();
    let end_y: usize = line.end_point.y.try_into().unwrap();
    let mut current_x: usize = start_x;
    let mut current_y: usize = start_y;
    loop {
        vent_map[current_x][current_y] += 1;
        if current_x == end_x && current_y == end_y {
            break;
        }

        if start_x > end_x {
            current_x -= 1;
        }

        if start_x < end_x {
            current_x += 1;
        }

        if start_y > end_y {
            current_y -= 1;
        }

        if start_y < end_y {
            current_y += 1;
        }
    }
}
