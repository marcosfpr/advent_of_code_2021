use std::{
    char::ToLowercase,
    collections::{HashMap, HashSet, VecDeque},
};

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> i32 {
    let octopuses: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let mut grid: Vec<Vec<i32>> = octopuses
        .iter()
        .map(|entry| entry.chars().map(|c| c.clone()).collect::<Vec<char>>())
        .map(|entry| {
            entry
                .iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    flash_octopuses(&mut grid, 100)
}


pub fn resolve_part_2(filename: &'static str) -> i32 {
  let octopuses: Vec<String> =
      aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut grid: Vec<Vec<i32>> = octopuses
      .iter()
      .map(|entry| entry.chars().map(|c| c.clone()).collect::<Vec<char>>())
      .map(|entry| {
          entry
              .iter()
              .map(|c| c.to_digit(10).unwrap() as i32)
              .collect::<Vec<i32>>()
      })
      .collect();

  flash_octopuses_broadcast(&mut grid)
}


fn flash_octopuses(grid: &mut Vec<Vec<i32>>, steps: u32) -> i32 {
    let mut flashes = VecDeque::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_flashes = 0;

    for _ in 0..steps {
        let mut has_flashed = Vec::new();

        // 1st: increases by 1 and add flashes to stack
        for i in 0..rows {
            for j in 0..cols {
                grid[i][j] += 1;
                if grid[i][j] > 9 {
                    flashes.push_back((i, j));
                }
            }
        }

        // 2nd: flashes the octopuses
        while let Some((i, j)) = flashes.pop_back() {
            if !has_flashed.contains(&(i, j)) {
                has_flashed.push((i, j));
            }
            else { continue;}

            let neighbors = get_neighbors(i as i32, j as i32, &grid);
            for neighbor in neighbors {
                grid[neighbor.0][neighbor.1] += 1;
                if grid[neighbor.0][neighbor.1] > 9 {
                    flashes.push_back(neighbor);
                }
            }
        }

        // 3rd: reset flashed octopuses
        for (i, j) in has_flashed {
            grid[i][j] = 0;
            total_flashes += 1;
        }
    }

    total_flashes
}


fn flash_octopuses_broadcast(grid: &mut Vec<Vec<i32>>) -> i32 {
  let mut flashes = VecDeque::new();

  let rows = grid.len();
  let cols = grid[0].len();

  let mut broadcast = false;
  let mut steps_to_broadcast = 0;

  while !broadcast{

      steps_to_broadcast += 1;
  
      let mut total_flashes = 0;
      let mut has_flashed = Vec::new();

      // 1st: increases by 1 and add flashes to stack
      for i in 0..rows {
          for j in 0..cols {
              grid[i][j] += 1;
              if grid[i][j] > 9 {
                  flashes.push_back((i, j));
              }
          }
      }

      // 2nd: flashes the octopuses
      while let Some((i, j)) = flashes.pop_back() {
          if !has_flashed.contains(&(i, j)) {
              has_flashed.push((i, j));
          }
          else { continue;}

          let neighbors = get_neighbors(i as i32, j as i32, &grid);
          for neighbor in neighbors {
              grid[neighbor.0][neighbor.1] += 1;
              if grid[neighbor.0][neighbor.1] > 9 {
                  flashes.push_back(neighbor);
              }
          }
      }

      // 3rd: reset flashed octopuses
      for (i, j) in has_flashed {
          grid[i][j] = 0;
          total_flashes += 1;
      }

      if total_flashes == rows * cols {
          broadcast = true;
      }
  }

  steps_to_broadcast
}


fn get_neighbors(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let neighbors = vec![
        (i - 1, j),     // up
        (i + 1, j),     // down
        (i, j - 1),     // left
        (i, j + 1),     // right
        (i - 1, j - 1), // up left
        (i - 1, j + 1), // up right
        (i + 1, j - 1), // down left
        (i + 1, j + 1), // right down
    ];

    let mut valid_neighbors: Vec<(usize, usize)> = neighbors
        .iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0)
        .map(|(i, j)| (*i as usize, *j as usize))
        .collect();

    valid_neighbors.retain(|(i, j)| grid.get(*i).and_then(|col| col.get(*j)).is_some());
    valid_neighbors
}
