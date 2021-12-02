mod aoc_ex_1;
mod aoc_ex_2;

use std::time::{Instant};

fn main() {
    
    let now = Instant::now();
    
    // Exercise 1
    // println!("Part 1: {}", aoc_ex_1::resolve_part_1("./inputs/input01.txt"));
    // println!("Part 2: {}", aoc_ex_1::resolve_part_2("./inputs/input01.txt"));

    // Exercise 2   
    println!("Part 1: {}", aoc_ex_2::resolve_part_1("./inputs/input02.txt"));   
    println!("Part 2: {}", aoc_ex_2::resolve_part_2("./inputs/input02.txt"));   

    println!("Time elapsed (s): {}", now.elapsed().as_millis());

}
