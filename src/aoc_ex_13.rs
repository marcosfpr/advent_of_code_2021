use std::{collections::HashMap, collections::HashSet, collections::VecDeque};

#[path = "aoc_utils.rs"]
mod aoc_utils;

#[derive(Debug)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}

pub fn resolve_part_1(filename: &'static str) -> i32 {
    // Read the input file  and parse it into a vector of strings
    let transparent_paper: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let (dots, folds) = read_coordinates(&transparent_paper);

    let mut new_dots = dots.clone();

    for fold in &folds {
        match fold {
            Instruction::FoldX(x) => {
                dots.iter().filter(|dot| dot.0 >= *x).for_each(|dot| {
                    let shift = dot.0 - *x;
                    new_dots.remove(&*dot);
                    new_dots.insert((*x - shift, dot.1));
                });
            }
            Instruction::FoldY(y) => {
                dots.iter().filter(|dot| dot.1 >= *y).for_each(|dot| {
                    let shift = dot.1 - *y;
                    new_dots.remove(&*dot);
                    new_dots.insert((dot.0, *y - shift));
                });
            }
        }
        return new_dots.len() as i32;
    }

    new_dots.len() as i32
}

pub fn resolve_part_2(filename: &'static str) -> String {
    // Read the input file  and parse it into a vector of strings
    let transparent_paper: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let (mut dots, folds) = read_coordinates(&transparent_paper);

    for fold in &folds {
        match fold {
            Instruction::FoldX(x) => {
                let folded_dots: HashSet<(usize, usize)> = dots
                    .iter()
                    .filter(|dot| dot.0 > *x)
                    .map(|(x, y)| (x.clone(), y.clone()))
                    .collect();
                folded_dots.iter().for_each(|dot| {
                    if let Some(new_x) = x.checked_sub(dot.0 - *x) {
                        dots.insert((new_x, dot.1));
                    }
                    dots.remove(&(dot.0, dot.1));
                });
            }
            Instruction::FoldY(y) => {
                let folded_dots: HashSet<(usize, usize)> = dots
                    .iter()
                    .filter(|dot| dot.1 > *y)
                    .map(|(x, y)| (x.clone(), y.clone()))
                    .collect();
                  
                folded_dots.iter().for_each(|dot| {
                    if let Some(new_y) = y.checked_sub(dot.1 - *y) {
                        dots.insert((dot.0, new_y));
                    }
                    dots.remove(&(dot.0, dot.1));
                });
            }
        }
    }

    let max_x = dots.iter().map(|dot| dot.0).max().unwrap();
    let max_y = dots.iter().map(|dot| dot.1).max().unwrap();

    let mut code = Vec::new();
    code.push("\n");

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if dots.contains(&(x, y)) {
                code.push("##");
            } else {
                code.push("  ");
            }
        }
        code.push("\n")
    }

    code.join("")
}

fn read_coordinates(
    transparent_paper: &Vec<String>,
) -> (HashSet<(usize, usize)>, Vec<Instruction>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    for line in transparent_paper {
        if line.contains("fold along") {
            let split_line = line.split(" ").last().unwrap();
            let split_fold = split_line.split("=").collect::<Vec<&str>>();
            if split_fold[0] == "x" {
                folds.push(Instruction::FoldX(split_fold[1].parse::<usize>().unwrap()));
            } else {
                folds.push(Instruction::FoldY(split_fold[1].parse::<usize>().unwrap()));
            }
        } else if line.contains(",") {
            let mut split_line = line.split(",");
            let x = split_line.next().unwrap().parse::<usize>().unwrap();
            let y = split_line.next().unwrap().parse::<usize>().unwrap();
            dots.insert((x, y));
        }
    }
    (dots, folds)
}
