#[path="aoc_utils.rs"]
mod aoc_utils;

use std::str;

pub fn resolve_part_1(filename: &'static str) -> u32 {
  let binaries: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut epsilon: String = String::new();
  let mut gamma: String = String::new();

  let bin_size = binaries[0].len();

  for j in 0..bin_size {
    
    let (ones, zeros) = count_bits(&binaries, j);

    if zeros > ones {
      epsilon.push('0');
      gamma.push('1');
    } else {
      epsilon.push('1');
      gamma.push('0');
    }
    

   }
  
  let gamma = binary_to_u32(gamma);
  let epsilon = binary_to_u32(epsilon);


  return gamma * epsilon;  

}

pub fn resolve_part_2(filename: &'static str) -> u32 {
  let mut binaries: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  binaries.sort();

  let oxygen_ratio = get_rating(&binaries, "1", &most_common_bit);
  let co2_ratio = get_rating(&binaries, "0", &least_common_bit);

  let oxygen_ratio = binary_to_u32(oxygen_ratio);
  let co2_ratio = binary_to_u32(co2_ratio);

  return oxygen_ratio * co2_ratio;
}

fn get_rating<'a>(binaries: &'a Vec<String>, tiebreak: &'a str, f: &dyn Fn(Vec<String>, usize, &'a str) -> &'a str ) -> String {
  let mut left: usize = 0;
  let mut right: usize = binaries.len() -1;
  let mut pos: usize = 0;

  while left < right {
    let keep_bit = f(binaries[left..right+1].to_vec(), pos, tiebreak);
    if keep_bit == "0" {
      while binaries[right].chars().nth(pos).unwrap() == '1' {
        right -= 1;
      }
    }
    else{
      while binaries[left].chars().nth(pos).unwrap() == '0' {
        left += 1;
      }
    }
    pos += 1;
  }

  binaries[right].clone()
}

fn count_bits(binaries: &Vec<String>, position: usize) -> (u32, u32) { 
  let mut zeros = 0;
  let mut ones = 0;

  for i in 0..binaries.len() {
    let mut number = binaries[i].chars();
    if number.nth(position).unwrap() == '0' {
      zeros += 1;
    } else {
      ones += 1;
    }
  }

  (zeros, ones)
}

fn most_common_bit<'a>(binaries: Vec<String>, position: usize, tiebreak: &'a str) -> &'a str {
  let (zeros, ones) = count_bits(&binaries, position);
  if zeros > ones {
    return "0";
  } else if ones > zeros {
    return "1";
  }
  tiebreak
}

fn least_common_bit<'a>(binaries: Vec<String>, position: usize, tiebreak: &'a str) -> &'a str {
  let (zeros, ones) = count_bits(&binaries, position);
  if zeros > ones {
    return "1";
  } else if ones > zeros {
    return "0";
  }
  tiebreak
}


fn binary_to_u32(s: String) -> u32 {
  let int_val = isize::from_str_radix(&s, 2).expect("Failed to convert to int");
  int_val as u32
}