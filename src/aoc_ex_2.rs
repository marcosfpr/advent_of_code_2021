#[path="aoc_utils.rs"]
mod aoc_utils;

use std::str;

pub fn resolve_part_1(filename: &'static str) -> u32 {
  
  let mut depth: u32 = 0;
  let mut forward: u32 = 0;

  for line  in aoc_utils::read_file_to_vec::<String>(filename).expect("Failed to read file") {
      let mut iter = line.split_whitespace();
      let command = iter.next().unwrap();
      let value = iter.next().unwrap().parse::<u32>().unwrap();

      if command == "up" {
        depth -= value;
      } else if command == "down" {
        depth += value;
      }
      else if command == "forward" {
        forward += value;
      }
  }
  
  forward * depth 
}


pub fn resolve_part_2(filename: &'static str) -> u64 {
  let mut depth: u32 = 0;
  let mut horizontal: u32 = 0;
  let mut aim: u32 = 0;

  for line  in aoc_utils::read_file_to_vec::<String>(filename).expect("Failed to read file") {
      let mut iter = line.split_whitespace();
      let command = iter.next().unwrap();
      let value = iter.next().unwrap().parse::<u32>().unwrap();

      if command == "up" {
        aim -= value;
      } else if command == "down" {
        aim += value;
      }
      else if command == "forward" {
        horizontal += value;
        depth += aim * value; 
      }
  }
  
  (horizontal * depth) as u64
}