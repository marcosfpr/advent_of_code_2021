use std::{collections::{HashMap, VecDeque, HashSet}, char::ToLowercase};

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> i32 {
  let navigation: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let chunk_open = vec!['{', '<', '(', '['];
  let chunk_close = vec!['}', '>', ')', ']'];

  let chunk_pair = chunk_open
    .iter()
    .zip(chunk_close.iter())
    .map(|(a, b)| (a.clone(), b.clone()))
    .collect::<HashMap<char, char>>();
  
  let mut illegal_points = 0;
  for line in &navigation {
    illegal_points += get_illegal_points(line, &chunk_pair);  
  }
  illegal_points
}


pub fn resolve_part_2(filename: &'static str) -> i64 {
  let navigation: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let chunk_open = vec!['{', '<', '(', '['];
  let chunk_close = vec!['}', '>', ')', ']'];

  let chunk_pair = chunk_open
    .iter()
    .zip(chunk_close.iter())
    .map(|(a, b)| (a.clone(), b.clone()))
    .collect::<HashMap<char, char>>();
  
  let mut autocomplete_points = Vec::new();

  for line in &navigation {
    let autocomplete =get_autocomplete_points(line, &chunk_pair);
    if autocomplete > 0 {
      autocomplete_points.push(autocomplete);
    }
  }

  autocomplete_points.sort();
  *autocomplete_points.get(autocomplete_points.len() / 2).unwrap()
}

fn  get_illegal_points(line: &String, chunk_pair: &HashMap<char, char>) -> i32 {
  let illegal_points_map:  HashMap<char, i32> = vec![(')', 3), (']',  57), ('}', 1197), ('>', 25137)].iter().cloned().collect();
  let close_map: HashMap<char,  char> = chunk_pair.iter().map(| (key, val) | (val.clone(), key.clone())).collect();
  
  let mut stack = VecDeque::new();

  for c in line.chars() {
    if chunk_pair.contains_key(&c) {
      stack.push_back(c);
    } else if close_map.contains_key(&c) {
      let popped = stack.pop_back().unwrap();
      if popped != *close_map.get(&c).unwrap() {
        return *illegal_points_map.get(&c).unwrap();
      }
    }
  }
  0
}

fn  get_autocomplete_points(line: &String, chunk_pair: &HashMap<char, char>) -> i64 {
  let autocomplete_points_map:  HashMap<char, i32> = vec![(')', 1), (']',  2), ('}', 3), ('>', 4)].iter().cloned().collect();

  let close_map: HashMap<char,  char> = chunk_pair.iter().map(| (key, val) | (val.clone(), key.clone())).collect();
  
  let mut stack = VecDeque::new();

  //  fill stack
  for c in line.chars() {
    if chunk_pair.contains_key(&c) {
      stack.push_back(c);
    } else if close_map.contains_key(&c) {

      let popped = stack.pop_back().unwrap();

      if popped != *close_map.get(&c).unwrap() {
        return 0; // illegal
      }
    }
  }

  let mut total_correction: i64 = 0;
  while let Some(c) = stack.pop_back() {
    let complete = chunk_pair.get(&c).unwrap();
    total_correction = total_correction  * 5 + (*autocomplete_points_map.get(complete).unwrap() as i64); 
  }

  total_correction
}
