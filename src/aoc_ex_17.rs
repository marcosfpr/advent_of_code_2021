use std::collections::HashSet;

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> i32 {
    // Read the input file  and parse it into a vector of strings
    let target_coordinates: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
    
    let target_coordinates = target_coordinates.iter().next().unwrap().clone();

    let start_end = target_coordinates.split("target area: ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>();

    let x_range = get_coordinate(start_end[0]);
    let y_range = get_coordinate(start_end[1]);
    
    
    // Max height happens when max velocity_y happens.
    let max_abs_velocity_y = std::cmp::max(y_range.0.abs() - 1,  y_range.1.abs());
    let max_y = max_abs_velocity_y * (max_abs_velocity_y + 1) / 2 as i32;
    
    max_y
}


pub fn resolve_part_2(filename: &'static str) -> i32 {
    // Read the input file  and parse it into a vector of strings
    let target_coordinates: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
    
    let target_coordinates = target_coordinates.iter().next().unwrap().clone();

    let start_end = target_coordinates.split("target area: ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>();

    let x_range = get_coordinate(start_end[0]);
    let y_range = get_coordinate(start_end[1]);
    
    let responses = test_launch_probe(2000, x_range, y_range);
    responses.len() as i32
}

fn get_coordinate(coord: &str) -> (i32, i32) {
    let splited = coord.split("=").collect::<Vec<&str>>()[1].split("..").collect::<Vec<&str>>();
    (splited[0].parse::<i32>().unwrap(), splited[1].parse::<i32>().unwrap())
}

fn test_launch_probe(k: i32, x_range: (i32, i32), y_range: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut response = HashSet::new();
    for i in -k..k {
        for j in -k..k {
            match launch_probe((i, j), x_range, y_range) {
                Some(_) => {  response.insert((i, j)); },
                None => continue,
            }
        }
    }
    response
}

fn launch_probe(velocity: (i32, i32), x_range: (i32, i32), y_range: (i32, i32)) -> Option<(i32, i32)> {
    let mut probe = (0, 0);
    let mut velocity = velocity;

    let mut highest_y = i32::MIN;
    let mut highest_x = i32::MIN;

    loop {
        // stop simulation if
        // probe is falling and target is above it
        if velocity.1 < 0 && probe.1 < y_range.0 {
            return None;
        }
        // velocity x converges to 0 and x=0 is not in target
        if velocity.0 == 0 && ! (probe.0 >= x_range.0 && probe.0 <= x_range.1) {
            return None;
        }
        // velocity is negative and probe is to the right/left
        if (velocity.0 < 0 && probe.0 < x_range.0) || (velocity.0 > 0 && probe.0 > x_range.1) {
            return None;
        }

        probe.0 += velocity.0;
        probe.1 += velocity.1;

        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        else if velocity.0 < 0 {
            velocity.0 += 1;
        }

        velocity.1 -= 1;

        if probe.0 > highest_x {
            highest_x = probe.0;
        }

        if probe.1 > highest_y {
            highest_y = probe.1;
        }

        // check boundaries of target
        if probe.0 >=  x_range.0 && probe.0 <= x_range.1 && probe.1 >= y_range.0 && probe.1 <= y_range.1 {
            return Some((highest_x, highest_y));
        }
        

    }
}