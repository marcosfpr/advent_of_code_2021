#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> usize {
    // Read the input file  and parse it into a vector of strings
    let hex_encrypted_message: Vec<String> =
        aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    // cast to binary
    let binary_encrypted_message: String =
        hex_to_binary(hex_encrypted_message.iter().next().unwrap());

    let mut version_ids = Vec::new();

    process(binary_encrypted_message.as_str(), &mut version_ids);

    // Return the sum of version ids 
    version_ids.iter().sum()
}


pub fn resolve_part_2(filename: &'static str) -> usize {
  // Read the input file  and parse it into a vector of strings
  let hex_encrypted_message: Vec<String> =
      aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

  // cast to binary
  let binary_encrypted_message: String =
      hex_to_binary(hex_encrypted_message.iter().next().unwrap());

  let mut version_ids = Vec::new();

  let response = process(binary_encrypted_message.as_str(), &mut version_ids);

  // Return the sum of version ids 
  response.unwrap().0
}

fn process(binary_msg: &str, version_ids: &mut Vec<usize>) -> Option<(usize, String)> {
    // three first bits: packet version
    let packet_version = usize::from_str_radix(&binary_msg[0..3], 2).unwrap();
    version_ids.push(packet_version);

    // next three bits: typeID
    let outer_type_id = usize::from_str_radix(&binary_msg[3..6], 2).unwrap();

    match outer_type_id {
        4 => process_literal(&binary_msg[6..]),
        i => process_operator(i, &binary_msg[6..], version_ids),
    }
}

fn process_operator(operator: usize, binary_msg: &str, version_ids: &mut Vec<usize>) -> Option<(usize, String)> {
    let operator_id = usize::from_str_radix(&binary_msg[0..1], 2).unwrap();
    let mut current_eval: Option<usize> = None;


    match operator_id {
        0 => {
            let bits_len = 15;

            let subpacket_len = usize::from_str_radix(&binary_msg[1..bits_len + 1], 2).unwrap();

            let start = bits_len + 1;
            let end = start + subpacket_len;

            let mut buffer = binary_msg[start..end].to_string();


            while buffer.len() != 0 {
                match process(&buffer, version_ids) {
                    Some((lit, left)) => {

                        buffer = left;

                        current_eval = Some(eval_operator(current_eval, lit, operator));
                        println!("New evaluation: {}", current_eval.unwrap());


                    }
                    None => {
                        return None;
                    }
                }
            }

            Some((current_eval.unwrap(), binary_msg[end..].to_string()))
        }
        1 => {
            let bits_len = 11;

            let number_subpackages =
                usize::from_str_radix(&binary_msg[1..bits_len + 1], 2).unwrap();

            let mut buffer = binary_msg[bits_len + 1..].to_string();

            for _ in 0..number_subpackages {
                match process(&buffer, version_ids) {
                    Some((lit, left)) => {

                        buffer = left;

                        current_eval = Some(eval_operator(current_eval, lit, operator));
                        println!("New evaluation: {}", current_eval.unwrap());

                    }
                    None => {
                        return None;
                    }
                }
            }

            Some((current_eval.unwrap(), buffer))
        }
        _ => panic!("Unknown operator"),
    }
}

fn process_literal(binary_msg: &str) -> Option<(usize, String)> {
    let mut i = 0;
    let mut number = String::new();

    loop {
        if binary_msg[i..].len() < 5 {
            return None;
        }

        let type_n = binary_msg.chars().nth(i).unwrap();
        let substr = &binary_msg[i..i + 5];
        number += &substr[1..];

        i += 5;

        if type_n == '0' {
            break;
        }
    }

    Some((
        usize::from_str_radix(&number, 2).unwrap(),
        binary_msg[i..].to_string(),
    ))
}

fn hex_to_binary(hex: &String) -> String {
    let mut binary_string = String::new();
    for c in hex.chars() {
        let binary_char = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Invalid hex character"),
        };
        binary_string.push_str(binary_char);
    }
    binary_string
}

fn eval_operator(current_eval: Option<usize>, lit: usize, operator: usize) -> usize {
  println!("Processing  [{}] between {} and {:?} ", operator, lit, current_eval);
  match operator {
    0 => current_eval.unwrap_or(0) + lit,
    1 => current_eval.unwrap_or(1) * lit,
    2 => std::cmp::min(current_eval.unwrap_or(usize::MAX), lit),
    3 => std::cmp::max(current_eval.unwrap_or(0), lit),
    5 => {
      match current_eval {
        Some(c) => if c > lit { 1 } else { 0 },
        None => lit
      }
    }
    6 => {
      match current_eval {
        Some(c) => if c < lit { 1 } else { 0 },
        None => lit
      }
    }
    7 => {
      match current_eval {
        Some(c) => if c == lit { 1 } else { 0 },
        None => lit
      }
    }
    _ => panic!("Unknown operator")
  }
  
}