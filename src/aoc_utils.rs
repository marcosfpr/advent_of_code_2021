use std::str;
use std::fs;

/**
 * T must be a type that implements the FromStr trait.
 */
pub fn read_file_to_vec<T: str::FromStr>(file_name: &str) -> Result<Vec<T>, <T as str::FromStr>::Err>
  where  T: str::FromStr,
         T: std::fmt::Debug {
  let mut file = fs::File::open(file_name).expect("File not found");
  let mut contents = String::new();
  std::io::Read::read_to_string(&mut file, &mut contents)
      .expect("Something went wrong reading the file");

  contents.lines()
    .map(|line| line.parse())
    .into_iter()
    .collect()
}

