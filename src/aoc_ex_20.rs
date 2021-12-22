use std::collections::HashSet;

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> usize {
    // Read the input file  and parse it into a vector of strings
    let trench_map: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let lookup: Vec<char> = trench_map[0]
        .clone()
        .trim()
        .chars()
        .collect();

    let mut image: HashSet<(i64, i64)> = HashSet::new();
    let mut rmin = i64::MAX;
    let mut rmax = i64::MIN;
    let mut cmin = i64::MAX;
    let mut cmax = i64::MIN;
    for (row, line) in trench_map[2..].iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                rmin = rmin.min(row as i64);
                rmax = rmax.max(row as i64);
                cmin = cmin.min(col as i64);
                cmax = cmax.max(col as i64);

                image.insert((row as i64, col as i64));
            }
        }
    }

    for step in 0..2 {
        let (exists_val, not_exists_val) = if step % 2 == 0 {
            ('1', '0')
        } else {
            ('0', '1')
        };

        let mut next_image: HashSet<(i64, i64)> = HashSet::new();
        let mut next_rmin = i64::MAX;
        let mut next_rmax = i64::MIN;
        let mut next_cmin = i64::MAX;
        let mut next_cmax = i64::MIN;

        for out_row in (rmin - 1)..=(rmax + 1) {
            for out_col in (cmin - 1)..=(cmax + 1) {
                let mut digits: Vec<char> = Vec::new();
                for i in (out_row - 1)..=(out_row + 1) {
                    for j in (out_col - 1)..=(out_col + 1) {
                        if image.contains(&(i, j)) {
                            digits.push(exists_val);
                        } else {
                            digits.push(not_exists_val);
                        }
                    }
                }

                let bin_str: String = digits.iter().collect();
                let index = usize::from_str_radix(&bin_str, 2).unwrap();

                let c = if step % 2 == 0 { '.' } else { '#' };
                if lookup[index] == c {
                    next_image.insert((out_row, out_col));

                    next_rmin = next_rmin.min(out_row as i64);
                    next_rmax = next_rmax.max(out_row as i64);
                    next_cmin = next_cmin.min(out_col as i64);
                    next_cmax = next_cmax.max(out_col as i64);
                }
            }
        }

        image = next_image;
        rmin = next_rmin;
        rmax = next_rmax;
        cmin = next_cmin;
        cmax = next_cmax;
    }

    image.len()
}


pub fn resolve_part_2(filename: &'static str) -> usize {
  // Read the input file  and parse it into a vector of strings
  let trench_map: Vec<String> =
      aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  let lookup: Vec<char> = trench_map[0]
      .clone()
      .trim()
      .chars()
      .collect();

  let mut image: HashSet<(i64, i64)> = HashSet::new();
  let mut rmin = i64::MAX;
  let mut rmax = i64::MIN;
  let mut cmin = i64::MAX;
  let mut cmax = i64::MIN;
  for (row, line) in trench_map[2..].iter().enumerate() {
      for (col, char) in line.chars().enumerate() {
          if char == '#' {
              rmin = rmin.min(row as i64);
              rmax = rmax.max(row as i64);
              cmin = cmin.min(col as i64);
              cmax = cmax.max(col as i64);

              image.insert((row as i64, col as i64));
          }
      }
  }

  for step in 0..50 {
      let (exists_val, not_exists_val) = if step % 2 == 0 {
          ('1', '0')
      } else {
          ('0', '1')
      };

      let mut next_image: HashSet<(i64, i64)> = HashSet::new();
      let mut next_rmin = i64::MAX;
      let mut next_rmax = i64::MIN;
      let mut next_cmin = i64::MAX;
      let mut next_cmax = i64::MIN;

      for out_row in (rmin - 1)..=(rmax + 1) {
          for out_col in (cmin - 1)..=(cmax + 1) {
              let mut digits: Vec<char> = Vec::new();
              for i in (out_row - 1)..=(out_row + 1) {
                  for j in (out_col - 1)..=(out_col + 1) {
                      if image.contains(&(i, j)) {
                          digits.push(exists_val);
                      } else {
                          digits.push(not_exists_val);
                      }
                  }
              }

              let bin_str: String = digits.iter().collect();
              let index = usize::from_str_radix(&bin_str, 2).unwrap();

              let c = if step % 2 == 0 { '.' } else { '#' };
              if lookup[index] == c {
                  next_image.insert((out_row, out_col));

                  next_rmin = next_rmin.min(out_row as i64);
                  next_rmax = next_rmax.max(out_row as i64);
                  next_cmin = next_cmin.min(out_col as i64);
                  next_cmax = next_cmax.max(out_col as i64);
              }
          }
      }

      image = next_image;
      rmin = next_rmin;
      rmax = next_rmax;
      cmin = next_cmin;
      cmax = next_cmax;
  }

  image.len()
}
