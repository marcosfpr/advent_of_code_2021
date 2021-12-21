#[path = "aoc_utils.rs"]
mod aoc_utils;


pub fn resolve_part_1(filename: &'static str) -> usize {
    // Read the input file  and parse it into a vector of strings
    let trench_map: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let img_enhancement: Vec<u8> = trench_map[0].clone().trim().chars().map(|c| if c == '.' { 0 } else { 1 }).collect();
    let (mut grid, mut boundaries) = read_grid(trench_map[1..].to_vec(), 250);

    for i in 0..2 {
      image_enhance(&mut grid, &mut boundaries, &img_enhancement, i);
      // // print grid for debugging purposes
      // for i in 0..grid.len() {
      //     for j in 0..grid[i].len() {
      //         print!("{}", if grid[i][j] == 0 { '.' } else { '#' });
      //     }
  
      //     println!();
      // } 
      
    }

    let  mut lit_points =  0;
    for  i in 0..grid.len(){
      for j in 0..grid[i].len(){
        if grid[i][j] == 1 {
          lit_points += 1;
        }
      }
    }
    lit_points
}

fn image_enhance(grid: &mut Vec<Vec<u8>>, boundaries: &mut [usize;4], img_enhancement: &Vec<u8>, step: usize) {
  let mut new_grid = vec![vec![0; grid.len()]; grid[0].len()];
  println!("{:?}", boundaries);

  let mut next_rmin = usize::MAX;
  let mut next_rmax = usize::MIN;
  let mut next_cmin = usize::MAX;
  let mut next_cmax = usize::MIN;

  let rmin = boundaries[0];
  let rmax = boundaries[1]; 
  let cmin = boundaries[2];
  let cmax = boundaries[3];

  for i in (rmin-1)..=(rmax+1) {
    for j in (cmin-1)..=(cmax+1) {
      let mask = get_from_binary(&grid, i, j);
      
      new_grid[i][j] = img_enhancement[mask as usize];
      if img_enhancement[mask as usize] == 1 {
        next_rmin = next_rmin.min(i);
        next_rmax = next_rmax.max(i);
        next_cmin = next_cmin.min(j);
        next_cmax = next_cmax.max(j);
      }
    }
  }
  *grid = new_grid;
  boundaries[0] = next_rmin;
  boundaries[1] = next_rmax;  
  boundaries[2] = next_cmin;
  boundaries[3] = next_cmax;
}

fn get_from_binary(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
  let mut binary: u32 = grid[i-1][j-1] as u32 *  2_u32.pow(8);
  binary += grid[i-1][j] as u32 *  2_u32.pow(7);
  binary += grid[i-1][j+1] as u32 *  2_u32.pow(6);
  binary += grid[i][j-1] as u32 *  2_u32.pow(5);
  binary += grid[i][j] as u32 *  2_u32.pow(4);
  binary += grid[i][j+1] as u32 *  2_u32.pow(3);
  binary += grid[i+1][j-1] as u32 *  2_u32.pow(2);
  binary += grid[i+1][j] as u32 *  2_u32.pow(1);
  binary += grid[i+1][j+1] as u32 *  2_u32.pow(0);  

  binary
}

fn read_grid(trench_map: Vec<String>, size: usize) -> (Vec<Vec<u8>>, [usize;4]) {
    let x_offset = (size - trench_map.len()) / 2 + 1;
    let y_offset = (size - trench_map[0].len()) / 2 + 1;


    let mut rmin = usize::MAX;
    let mut rmax = usize::MIN;
    let mut cmin = usize::MAX;
    let mut cmax = usize::MIN;

    // 510 e 10x10
    // x_offset = (510 - 10) / 2 =  250
    // y_offset = (510 - 10) / 2 =  250

    let mut grid: Vec<Vec<u8>> = vec![vec![0; size]; size];

    for (x, row) in trench_map.iter().enumerate() {
        for (y, c) in row.chars().enumerate() {
            grid[x+x_offset][y+y_offset] = if c == '.' { 0 } else { 1 };
            if c == '#' { 
              rmin = rmin.min(x+x_offset);
              rmax = rmax.max(x+x_offset);
              cmin = cmin.min(y+y_offset);
              cmax = cmax.max(y+y_offset);
            }
        }
    }

    (grid, [rmin, rmax, cmin, cmax])
} 
