#[path="aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> i32 {
  let crabs: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let mut crabs: Vec<i32>  = crabs.iter().next()
  .unwrap().split(",").map(|x| x.trim().parse::<i32>().unwrap())
  .collect();

  crabs.sort();

  let min = crabs[0];
  let max = crabs.last().unwrap();

  let mut min_fuel = fuel_spent(min, &crabs, -1).unwrap();

  for i in (min+1)..(max+1) {
    if let Some(fuel) =  fuel_spent(i, &crabs, min_fuel) {
      min_fuel = fuel;
    }
  }

  min_fuel

}


pub fn resolve_part_2(filename: &'static str) -> i32 {
  let crabs: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let mut crabs: Vec<i32>  = crabs.iter().next()
  .unwrap().split(",").map(|x| x.trim().parse::<i32>().unwrap())
  .collect();

  crabs.sort();

  let min = crabs[0];
  let max = crabs.last().unwrap();

  let mut min_fuel = fuel_spent_2(min, &crabs, -1).unwrap();

  for i in (min+1)..(max+1) {
    if let Some(fuel) =  fuel_spent_2(i, &crabs, min_fuel) {
      min_fuel = fuel;
    }
  }

  min_fuel

}


fn fuel_spent(horizontal_position: i32, crabs: &Vec<i32>, bound: i32) -> Option<i32> {
  let mut total_fuel = 0;

  for crab in crabs {
    total_fuel += (crab - horizontal_position).abs();
    if bound > 0 && total_fuel > bound {
      return None
    }
  }

  Some(total_fuel)
}

fn fuel_spent_2(horizontal_position: i32, crabs: &Vec<i32>, bound: i32) -> Option<i32> {
  let mut total_fuel = 0;

  for  crab in crabs{
    let shifts = (crab - horizontal_position).abs();
    total_fuel += (shifts * (shifts + 1)) / 2;
    if bound > 0 && total_fuel > bound {
      return None
    }
  }

  Some(total_fuel)
}