#[path="aoc_utils.rs"]
mod aoc_utils;

use std::{str, collections::HashMap};

enum Shift {
  Horizontal,
  Vertical,
  DiagonalUp,
  DiagonalDown,
  None
}

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]    
struct Line {
  pub begin: (i32, i32),
  pub end: (i32, i32)  
}

pub fn resolve_part_1(filename: &'static str) -> u32 {
  let coordinates_str: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  let coordinates: Vec<Line> = read_coordinates(coordinates_str);

  let mut mapping_vents: HashMap<(i32, i32), u32> = HashMap::new();
  
  for line in &coordinates {

    //  direction i'll go through
    let shift = match get_shift(line) {
      Shift::Vertical => (0, 1),
      Shift::Horizontal => (1, 0),
      _ => continue,
    };


    let mut pos = line.begin;
    mapping_vents.insert(pos, mapping_vents.get(&pos).unwrap_or(&0) + 1); 

    while pos != line.end {
      pos = (pos.0 + shift.0, pos.1 + shift.1);
      mapping_vents.insert(pos, mapping_vents.get(&pos).unwrap_or(&0) + 1);
    }
  }

  mapping_vents.values().filter(|&x| *x > 1).count() as u32

}

pub fn resolve_part_2(filename: &'static str) -> u32 {
  let coordinates_str: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  let coordinates: Vec<Line> = read_coordinates(coordinates_str);

  let mut mapping_vents: HashMap<(i32, i32), u32> = HashMap::new();
  
  for line in &coordinates {

    //  direction i'll go through
    let shift: (i32, i32) = match get_shift(line) {
      Shift::Vertical => (0, 1),
      Shift::Horizontal => (1, 0),
      Shift::DiagonalUp => (1, 1),
      Shift::DiagonalDown => (1, -1),
      _ => continue,
    };

    println!("{:?} with shift {:?}", line, shift);


    let mut pos = line.begin;
    mapping_vents.insert(pos, mapping_vents.get(&pos).unwrap_or(&0) + 1); 

    while pos != line.end {
      pos = (pos.0 + shift.0, pos.1 + shift.1);
      mapping_vents.insert(pos, mapping_vents.get(&pos).unwrap_or(&0) + 1);
    }
  }

  mapping_vents.values().filter(|&x| *x > 1).count() as u32

}


fn get_shift(line: &Line) -> Shift {
  if line.begin.0 == line.end.0 {
    Shift::Vertical
  }
  else if line.begin.1 == line.end.1 {
    Shift::Horizontal
  }
  else if line.begin.1 < line.end.1{
    Shift::DiagonalUp
  }
  else if line.begin.1 > line.end.1{
    Shift::DiagonalDown
  }
  else {
    Shift::None
  }

}

fn read_coordinates(raw_str: Vec<String>) -> Vec<Line> {
  let mut coordinates: Vec<Line> = Vec::new();
  for line in raw_str {
    let mut line_splited = line.split("->");
    let begin: Vec<i32> = line_splited.next()
      .unwrap().split(",").map(|x| x.trim().parse::<i32>().unwrap())
      .collect();

    let end: Vec<i32> = line_splited.next()
      .unwrap().split(",").map(|x| x.trim().parse::<i32>().unwrap())
      .collect();

    insert_line(begin[0], begin[1], end[0], end[1], &mut coordinates);
  }
  coordinates.sort();

  coordinates
}

fn insert_line(x1: i32, y1: i32, x2: i32, y2: i32, coordinates: &mut Vec<Line>) {

  // for sorting  good.
  if x1 > x2 || y1 > y2{

    if x1 < x2 && y1 > y2 {
      coordinates.push(Line {
        begin: (x1, y1),
        end: (x2, y2)
      });
    }
    else {
      coordinates.push(Line {
        begin: (x2, y2),
        end: (x1, y1)
      });
    }
  }
  else {
    coordinates.push(Line {
      begin: (x1, y1),
      end: (x2, y2)
    });
  }
} 