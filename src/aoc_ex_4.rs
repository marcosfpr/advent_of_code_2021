#[path="aoc_utils.rs"]
mod aoc_utils;

use std::{str, collections::{HashSet, LinkedList}};


pub type Bingo = HashSet<u32>;
pub type Row = Vec<u32>;
pub type Matrix = Vec<Row>;

#[derive(Debug, PartialEq, Eq)]
struct Board {
  win_rows: Vec<Bingo>  
}

impl Board {
  fn new(matrix: &Matrix) -> Board {
    let mut win_rows = Vec::new();

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
      let mut row: Bingo = HashSet::new();
      for j in 0..cols {
        row.insert(matrix[i][j]);
      }
      win_rows.push(row);
    }

    for j in 0..cols {
      let mut row: Bingo = HashSet::new();
      for i in 0..rows {
        row.insert(matrix[i][j]);
      }
      win_rows.push(row);
    }

    
    Board { win_rows }
  }

  fn has_won(&self) -> bool {
    for row in &self.win_rows {
      if row.len() == 0 {
        return true;
      }
    }
    false
  }

  fn mark_item(&mut self, item: u32) {
    for row in &mut self.win_rows {
      row.remove(&item);
    }
  }

  fn unmark_sum(&self) -> u32 {
    let mut sum = 0;
    for row in &self.win_rows {
      for item  in row {
        sum += item;
      }
    }
    sum / 2
  }

}


pub fn resolve_part_1(filename: &'static str) -> u32 {
  let bingo: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut board_iterator = bingo.iter();

  // first line: sorted values
  let items: Vec<u32>  = board_iterator.next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect();
  
  board_iterator.next(); // skip empty line

  let mut boards: Vec<Board> = get_bingo_boards(board_iterator, 5);

  for item in items {
    
    for board in &mut boards {
      board.mark_item(item);
      
      if board.has_won() {
        return board.unmark_sum() * item;
      }

    }

  }

  0
}


pub fn resolve_part_2(filename: &'static str) -> u32 {
  let bingo: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut board_iterator = bingo.iter();

  // first line: sorted values
  let items: Vec<u32>  = board_iterator.next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect();
  
  board_iterator.next(); // skip empty line

  let mut boards: Vec<Board> = get_bingo_boards(board_iterator, 5);

  let mut win_order: LinkedList<u32> = LinkedList::new();
  for item in items {
    
    for board in &mut boards {
      if board.has_won() {
        continue;
      }

      board.mark_item(item);

      if board.has_won() {
        win_order.push_back(board.unmark_sum() * item);
      }

    }

  }

  win_order.pop_back().unwrap()
}




fn get_bingo_boards(mut board_iterator: core::slice::Iter<String>, size: usize) -> Vec<Board> {
  let mut boards: Vec<Board> = Vec::new();
  // second line: boards
  loop {
    let mut board: Matrix = Vec::new();
    for _ in 0..size {
      let raw_line = board_iterator.next();
      match raw_line {
        Some(line) => {
          let row: Row = line.trim().split(char::is_whitespace)
            .filter(|x| !x.is_empty())
            .map(|x|  x.parse::<u32>().unwrap()).collect();

          if !row.is_empty() {
            board.push(row);
          }
        },
        None => {
          return boards;
        }
      }
    }
    boards.push(Board::new(&board));
    board_iterator.next(); // skip empty line 
  }
}