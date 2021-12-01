#[path="aoc_utils.rs"]
mod aoc_utils;

use std::str;


pub fn resolve_part_1(filename: &'static str) -> u32 {

  let depths: Vec<u32> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut count_increases: u32 = 0;

  for i in 1..depths.len() {
    
   if  depths[i-1] < depths[i] {
      count_increases += 1;
    }
    
  }

  count_increases
}

pub fn resolve_part_2(filename: &'static str) -> u32 {

  let depths: Vec<u32> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut count_increases: u32 = 0;

  // sliding window of size 3
  for i in 1..(depths.len() - 2) {

    let current  = depths[i-1] + depths[i] + depths[i+1];
    let next = depths[i] + depths[i+1] + depths[i+2];

    if next > current {
      count_increases += 1;
    }
  }
  
  count_increases
}