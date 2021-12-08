#[path="aoc_utils.rs"]
mod aoc_utils;

use std::{str, collections::{LinkedList, VecDeque}};

// slow
pub fn resolve_part_1(filename: &'static str, days: u32) -> u64 {
  let lanternfishs: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let mut counter: LinkedList<u32>  = lanternfishs.iter().next()
  .unwrap().split(",").map(|x| x.trim().parse::<u32>().unwrap())
  .collect();


  for _ in 0..days {

    let mut new_lanternfishs: LinkedList<u32> = LinkedList::new();
    counter.iter_mut().for_each(|x| {
      if *x == 0 {
        *x = 6;
        new_lanternfishs.push_back(8);
      }
      else {
        *x -= 1;
      }
    });

    counter.append(&mut new_lanternfishs);
    
  }

  counter.len() as u64
}

// fast
pub fn resolve_part_2(filename: &'static str, days: u32) -> u64 {
  let lanternfishs: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let counter: Vec<u32>  = lanternfishs.iter().next()
  .unwrap().split(",").map(|x| x.trim().parse::<u32>().unwrap())
  .collect();

  // key: days left, value: number of lanternfishs
  let mut days_counter: VecDeque<u64> = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);   
  for i in 0..9 {
    days_counter[i] = counter.iter().filter(|x| **x == i as u32).count() as u64;
  }
  
  for _ in 0..days {
    days_counter[7] += days_counter[0];
    days_counter.rotate_left(1);
  }

  let s = days_counter.iter().sum();
  s
}