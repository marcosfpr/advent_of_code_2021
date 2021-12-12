use std::{collections::HashMap, collections::HashSet, collections::VecDeque};

#[path = "aoc_utils.rs"]
mod aoc_utils;

type Path = VecDeque<String>;

pub fn resolve_part_1(filename: &'static str) -> i32 {
    let edges: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for edge in edges {
        let mut split_edge = edge.split("-");

        let first_node = split_edge.next().unwrap().to_string();
        let second_node = split_edge.next().unwrap().to_string();

        graph
            .entry(first_node.clone())
            .or_insert(HashSet::new())
            .insert(second_node.clone());
        graph
            .entry(second_node)
            .or_insert(HashSet::new())
            .insert(first_node);
    }

    paths_small_once(&graph)
}

pub fn resolve_part_2(filename: &'static str) -> i32 {
  let edges: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

  for edge in edges {
      let mut split_edge = edge.split("-");

      let first_node = split_edge.next().unwrap().to_string();
      let second_node = split_edge.next().unwrap().to_string();

      graph
          .entry(first_node.clone())
          .or_insert(HashSet::new())
          .insert(second_node.clone());
      graph
          .entry(second_node)
          .or_insert(HashSet::new())
          .insert(first_node);
  }

  paths_small_twice(&graph)
}

fn paths_small_once(graph: &HashMap<String, HashSet<String>>) -> i32 {
    let start_node = "start".to_string();

    let mut queue: VecDeque<Path> = VecDeque::new();

    let mut paths = 0;

    graph.get(&start_node).unwrap().iter().for_each(|node| {
        queue.push_back(Path::from(vec![start_node.clone(), node.clone()]));
    });

    while let Some(path) = queue.pop_front() {
        // check if i'm on the end
        let end_node = path.back().unwrap();
        if end_node == "end" {
            paths += 1;
            continue;
        }

        graph.get(end_node).unwrap().iter().for_each(|neighbor| {
            if neighbor.chars().next().unwrap().is_uppercase() {
                // big cave
                let mut new_path = path.clone();
                new_path.push_back(neighbor.clone());
                queue.push_back(new_path);
            } else {
                if !path.contains(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push_back(neighbor.clone());
                    queue.push_back(new_path);
                }
            }
        });
    }

    paths
}


fn paths_small_twice(graph: &HashMap<String, HashSet<String>>) -> i32 {
  let start_node = "start".to_string();

  // path, small cave repeated
  let mut queue: VecDeque<(Path, bool)> = VecDeque::new();

  let mut paths = 0;

  graph.get(&start_node).unwrap().iter().for_each(|node| {
      queue.push_back((Path::from(vec![start_node.clone(), node.clone()]), false));
  });

  let mut i = 1;
  while let Some((path, repeated)) = queue.pop_front() {
      // check if i'm on the end
      let end_node = path.back().unwrap();
      if end_node == "end" {
          paths += 1;
          continue;
      }

      graph.get(end_node).unwrap().iter().for_each(|neighbor| {
          // if neighbor is a big cave, i can repeat it
          if neighbor.chars().next().unwrap().is_uppercase() {
              // big cave
              let mut new_path = path.clone();
              new_path.push_back(neighbor.clone());
              queue.push_back((new_path, repeated));
          } else { 
              // if neighbor is a small cave, i can repeat it only if i haven't already visited it
              if !path.contains(neighbor) {
                  let mut new_path = path.clone();
                  new_path.push_back(neighbor.clone());
                  queue.push_back((new_path, repeated));
              }
              else { 
                if !repeated && neighbor != "start" && neighbor != "end" { // or if i don't visit a small cave twice yet
                  let mut new_path = path.clone();
                  new_path.push_back(neighbor.clone());
                  queue.push_back((new_path, true));
              
                }
              }
          }
      });
  }

  paths
}
