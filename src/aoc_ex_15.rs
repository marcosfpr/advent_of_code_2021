use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

#[path = "aoc_utils.rs"]
mod aoc_utils;

type Pos = (usize, usize);

pub fn resolve_part_1(filename: &'static str) -> usize {
    // Read the input file  and parse it into a vector of strings
    let risk_level: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let risk_level: Vec<Vec<usize>> = risk_level
        .iter()
        .map(|entry| {
            entry
                .trim()
                .chars()
                .map(|c| char::to_digit(c, 10).unwrap() as usize)
                .collect()
        })
        .collect();

    let end = risk_level.len() - 1;
    dijkstra(&risk_level, (0, 0), (end, end),  risk_level.len() as i32).unwrap()
}


pub fn resolve_part_2(filename: &'static str) -> usize {
  // Read the input file  and parse it into a vector of strings
  let risk_level: Vec<String> =
      aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let cave: Vec<Vec<usize>> = risk_level
      .iter()
      .map(|entry| {
          entry
              .trim()
              .chars()
              .map(|c| char::to_digit(c, 10).unwrap() as usize)
              .collect()
      })
      .collect();

      
  let multiplier: i32 = 5;
  let cave_size = cave.len();
  let new_cave_size = cave_size * multiplier as usize;

  let new_start = (0,0);
  let new_end = (new_cave_size - 1, new_cave_size - 1);

  let mut new_cave: Vec<Vec<usize>> = vec![vec![0; new_cave_size as usize]; new_cave_size as usize];
  
  for tile_x in 0..multiplier {
    for tile_y in 0..multiplier {
      for i in 0..cave_size {
        for j in 0..cave_size  {
          let new_risk_level = (cave[i][j] as i32 + tile_x  + tile_y - 1) % 9 + 1;

          let new_i = tile_x as usize * cave_size + i;
          let new_j = tile_y as usize * cave_size + j; 
          
          new_cave[new_i][new_j] = new_risk_level as usize;  
        }   
      }
    }
  }
    
  dijkstra(&Vec::from(new_cave), new_start, new_end,  new_cave_size as i32).unwrap()
}

fn dijkstra(matrix: &Vec<Vec<usize>>, start: Pos, end: Pos, cave_size: i32) -> Option<usize> {

    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, start)));

    while !heap.is_empty() {
        let (cost, cur) = heap.pop().unwrap().0;

        if cur == end {
            return Some(cost);
        }

        let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions.iter() {
            let next_x = cur.0 as i32 + dx;
            let next_y = cur.1 as i32 + dy;
            let next = (next_x, next_y);
            
            //  sanity check
            if !(next.0 >= 0 && next.0 < cave_size && next.1 >= 0 && next.1 < cave_size) {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);
            let next_cost = cost  + matrix[next.0][next.1];

            if costs.contains_key(&next) && *costs.get(&next).unwrap() <= next_cost {
                continue;
            }

            *costs.entry(next).or_insert(next_cost) = next_cost;
            heap.push(Reverse((next_cost, next)));
        }
    }

    None
}
