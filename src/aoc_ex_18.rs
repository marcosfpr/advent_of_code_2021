use std::io::{self, Read};


#[path = "aoc_utils.rs"]
mod aoc_utils;


#[derive(Debug, Clone)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>
}

impl VecTree {
    fn parse(s: &str) -> VecTree {
        let mut tree =  VecTree {
            vals: Vec::new(),
            depths: Vec::new()
        };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    tree.vals.push(d.to_digit(10).unwrap());
                    tree.depths.push(depth-1);
                }
            }
        }

        tree
    }


    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn split(&mut self) -> bool {
        for i in 0..self.vals.len() {
            let v = self.vals[i];
            if v < 10 {
                continue;
            }

            let (a,b)  = if v % 2 == 0 {
                (v/2, v/2)
            } else {
                (v/2, v/2 + 1)
            };

            self.vals[i] = a;
            self.depths[i] += 1;
            self.vals.insert(i+1, b);
            self.depths.insert(i+1, self.depths[i]);    

            return true;
        }
        false
    }

    fn explode(&mut self) -> bool  {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth != 4  {
                continue;
            }

            // add  left value to left neighbor
            if i != 0 {
                self.vals[i-1] += self.vals[i];
            }

            // add right value to right neighbor
            if i + 2 < self.vals.len() {
                self.vals[i+2] += self.vals[i+1];
            }

            self.vals[i] = 0;
            self.depths[i] = 3;
            self.vals.remove(i+1);  // remove right value
            self.depths.remove(i+1);  // remove right depth

            return true;
        }

        false
    }

    fn add(&mut self, other: &VecTree ) {
        self.vals.extend(other.vals.iter());
        self.depths.extend(other.depths.iter());    
        for i in 0..self.depths.len() {
            self.depths[i] += 1;
        }
    }

    fn score(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut depths =  self.depths.clone();  

        while  vals.len() > 1 {
            for i in 0..depths.len() - 1 {
                if depths[i] == depths[i+1] {
                    vals[i] = 3* vals[i] + 2 * vals[i+1];
                    vals.remove(i+1);
                    depths.remove(i+1);

                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    break;
                }
            }
        }

        vals[0]
    }
}

pub fn resolve_part_1(filename: &'static str) -> u32 {
    // Read the input file  and parse it into a vector of strings
    let snailfish_numbers: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let snail_tree: Vec<VecTree> = snailfish_numbers.iter().map(|s| VecTree::parse(s)).collect();

    let mut tree = snail_tree[0].clone();
    for other in snail_tree[1..].iter() {
        tree.add(other);
        tree.reduce();
    }      

    tree.score()
  
}

pub fn resolve_part_2(filename: &'static str) -> u32 {
    // Read the input file  and parse it into a vector of strings
    let snailfish_numbers: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let snail_tree: Vec<VecTree> = snailfish_numbers.iter().map(|s| VecTree::parse(s)).collect();

    let mut best_score = 0;
    for tree in snail_tree.iter() {
        for other in snail_tree.iter() {
            let mut a = tree.clone();
            a.add(other);
            a.reduce();
            best_score = std::cmp::max(best_score, a.score());
        }
    }

    best_score  
  
}