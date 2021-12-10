mod aoc_ex_1;
mod aoc_ex_2;
mod aoc_ex_3;
mod aoc_ex_4;
mod aoc_ex_5;
mod aoc_ex_6;
mod aoc_ex_7;
mod aoc_ex_8;
mod aoc_ex_9;

use std::time::{Instant};

fn main() {
    
    let now = Instant::now();
    
    // Exercise 1
    // println!("Part 1: {}", aoc_ex_1::resolve_part_1("./inputs/input01.txt"));
    // println!("Part 2: {}", aoc_ex_1::resolve_part_2("./inputs/input01.txt"));

    // Exercise 2   
    // println!("Part 1: {}", aoc_ex_2::resolve_part_1("./inputs/input02.txt"));   
    // println!("Part 2: {}", aoc_ex_2::resolve_part_2("./inputs/input02.txt"));  
    
    // Exercise 3   
    // println!("Part 1: {}", aoc_ex_3::resolve_part_1("./inputs/input03.txt"));   
    // println!("Part 2: {}", aoc_ex_3::resolve_part_2("./inputs/input03.txt"));

    // Exercise 4
    // println!("Part 1: {}", aoc_ex_4::resolve_part_1("./inputs/input04.txt"));
    // println!("Part 2: {}", aoc_ex_4::resolve_part_2("./inputs/input04.txt"));

    //Exercise 5
    // println!("Part 1: {}", aoc_ex_5::resolve_part_1("./inputs/input05.txt"));
    // println!("Part 2: {}", aoc_ex_5::resolve_part_2("./inputs/input05.txt"));

    //Exercise 6
    // println!("Part 1: {}", aoc_ex_6::resolve_part_1("./inputs/input06.txt",  80));
    // println!("Part 2: {}", aoc_ex_6::resolve_part_2("./inputs/input06.txt",  256));

    //Exercise 7
    // println!("Part 1: {}", aoc_ex_7::resolve_part_1("./inputs/input07.txt"));
    // println!("Part 2: {}", aoc_ex_7::resolve_part_2("./inputs/input07.txt"));

    //Exercise 8
    // println!("Part 1: {}", aoc_ex_8::resolve_part_1("./inputs/input08.txt"));
    // println!("Part 2: {}", aoc_ex_8::resolve_part_2("./inputs/input08.txt"));
    
    //Exercise 9
    // println!("Part 1: {}", aoc_ex_9::resolve_part_1("./inputs/input09.txt"));
    println!("Part 2: {}", aoc_ex_9::resolve_part_2("./inputs/input09.txt"));

    println!("Time elapsed (s): {}", now.elapsed().as_millis());

}
