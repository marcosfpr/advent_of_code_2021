// credits to https://github.com/jeffomatic

use std::{collections::{HashSet, HashMap}, ops::{Add, Sub, Neg}};

#[path = "aoc_utils.rs"]
mod aoc_utils;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Neg  for Vec3 {
  type Output = Self;
  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  } 
}


impl Vec3 {
  fn  zero() -> Self {
    Self {
      x: 0,
      y: 0,
      z: 0,
    }
  }

  fn rotate(self, axis_rotations: Self) -> Self {
    let mut new_vec = self;
    
    // x-axis rotation
    for _ in 0..axis_rotations.x {
      let prev_z = new_vec.z;
      new_vec.z = new_vec.y;    
      new_vec.y = -prev_z;
    }

    // y-axis rotation
    for _ in 0..axis_rotations.y {
      let prev_z = new_vec.z;
      new_vec.z = -new_vec.x;
      new_vec.x = prev_z;
    }

    // z-axis rotation
    for _ in 0..axis_rotations.z {
      let prev_y = new_vec.y;
      new_vec.y = new_vec.x;
      new_vec.x = -prev_y;
    }

    new_vec
  }

}

fn translate_points(src: &HashSet<Vec3>, translate: Vec3) -> HashSet<Vec3> {
  src.iter().map(|p| *p + translate).collect()
}

fn find_offset(a: &HashSet<Vec3>, b: &HashSet<Vec3>, min_similar: usize) -> Option<Vec3> {
  for a_origin in a.iter() {
      let a_translated = translate_points(a, -*a_origin);
      for b_origin in b.iter() {
          let b_translated = translate_points(b, -*b_origin);
          if a_translated.intersection(&b_translated).count() >= min_similar {
              return Some(*a_origin - *b_origin);
          }
      }
  }

  None
}


pub fn resolve_part_1(filename: &'static str) -> i32 {
    // Read the input file  and parse it into a vector of strings
    let scanners_coordinates: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let scanners = read_coordinates(scanners_coordinates);

    get_beacons_and_distance(scanners).0.len() as i32 
}

pub fn resolve_part_2(filename: &'static str) -> i32 {
  // Read the input file  and parse it into a vector of strings
  let scanners_coordinates: Vec<String> =
      aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let scanners = read_coordinates(scanners_coordinates);

  get_beacons_and_distance(scanners).1
}



fn get_beacons_and_distance(scanners: Vec<HashMap<Vec3, HashSet<Vec3>>>) -> (HashSet<Vec3>, i32){
  let min_overlap = 12;

  // scanner_id -> beacon coordinate mapped to scanner 0
  let mut resolved = HashMap::new();
  resolved.insert(0, scanners[0][&Vec3::zero()].clone());

  let mut offsets = HashMap::new();
  offsets.insert(0, Vec3::zero());

  let mut unresolved = HashSet::new();
  for i in 1..scanners.len() {
    unresolved.insert(i);
  }

  let mut unrelated_scanners = HashSet::new();


  'outer: while unresolved.len() > 0 {
    for a_id in resolved.keys().cloned() {
      let a_points = &resolved[&a_id];
      for b_id in unresolved.clone() {
        if unrelated_scanners.contains(&(a_id, b_id)) {
          continue;
        }
        for b_points in scanners[b_id].values() {
          if let Some(b_to_a) = find_offset(a_points, b_points, min_overlap) {
            unresolved.remove(&b_id);
            resolved.insert(b_id, translate_points(b_points, b_to_a));
            offsets.insert(b_id, b_to_a);
            continue 'outer;
          }
        }

        unrelated_scanners.insert((a_id, b_id));
      }
    }

    unreachable!();
  }

  let mut beacons = HashSet::new();
  for scanner_beacons in resolved.values() {
      beacons = beacons.union(scanner_beacons).map(|v| *v).collect();
  }

  let offsets: Vec<Vec3> = offsets.values().map(|v| *v).collect();
  let mut best = 0;
  for i in 0..offsets.len() - 1 {
      for j in (i + 1)..offsets.len() {
          let d = offsets[j] - offsets[i];
          best = best.max(d.x.abs() + d.y.abs() + d.z.abs());
      }
  }

  (beacons, best)
}


fn all_rotations() -> Vec<Vec3> {
  let mut response = Vec::new();
  let mut known_rots = HashSet::new();

  for xrot in 0..=3 {
    for yrot in 0..=3 {
      for zrot in 0..=3 {
        let rot = Vec3 {x: xrot, y: yrot, z: zrot};

        let axes_rotated = (
          Vec3 { x: 1, y: 0, z: 0}.rotate(rot),
          Vec3 { x: 0, y: 1, z: 0}.rotate(rot),
          Vec3 { x: 0, y: 0, z: 1}.rotate(rot), 
        );

        if known_rots.contains(&axes_rotated) {
          continue;
        }

        known_rots.insert(axes_rotated);
        response.push(rot);  
      }
    }
  }

  response
}


fn read_coordinates(scanners_coordinates: Vec<String> ) -> Vec<HashMap<Vec3, HashSet<Vec3>>>{
  let mut scanners = Vec::new();
  let mut lines = scanners_coordinates.iter().peekable();

  let all_rots = all_rotations();

  while lines.peek().is_some() {
      lines.next(); // consume hyphen

      let mut points = HashSet::new();
      loop {
        if lines.peek().is_none() {
          break;
        }
        let line = lines.next().unwrap(); 
        if line.len() == 0 {
          break;
        }

        let coords: Vec<i32> = line.split(",").map(|toks| toks.parse::<i32>().unwrap()).collect();
        points.insert(Vec3 {x: coords[0], y: coords[1], z: coords[2]});
      }

      let mut rots: HashMap<Vec3, HashSet<Vec3>> =  HashMap::new();
      for r in all_rots.iter() {

        rots.insert(*r, points.iter().map(|p| p.rotate(*r)).collect());
      }

      scanners.push(rots);
  }

  scanners
}