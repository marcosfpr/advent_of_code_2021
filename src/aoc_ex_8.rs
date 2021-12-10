use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

type FrozenSet<T> = BTreeSet<T>;

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub fn resolve_part_1(filename: &'static str) -> i32 {
    let codes: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let entries = read_entries(&codes);

    let mut total_easy_numbers = 0;

    for (entry_in, entry_out) in entries {
        let decoded_entry = decode_easy(&entry_in, &entry_out);

        let easy_numbers = String::from("1478");

        total_easy_numbers += decoded_entry
            .split(" ")
            .filter(|number| easy_numbers.contains(number))
            .count() as i32
    }

    total_easy_numbers
}

pub fn resolve_part_2(filename: &'static str) -> i32 {
    let codes: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let entries = read_entries(&codes);

    let mut total_sum = 0;

    for (entry_in, entry_out) in entries {
        let decoded_entry = decode(&entry_in, &entry_out);
        println!("{}", decoded_entry);
        total_sum += decoded_entry
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }

    total_sum
}

fn read_entries(codes: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut all_entries: LinkedList<(Vec<String>, Vec<String>)> = LinkedList::new();

    for line in codes {
        let mut entries = line.split(" | ");

        let entry_in = entries
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<String>().unwrap())
            .collect();
        let entry_out = entries
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<String>().unwrap())
            .collect();

        all_entries.push_back((entry_in, entry_out));
    }

    all_entries.into_iter().collect()
}

fn decode_easy(entry_in: &Vec<String>, entry_out: &Vec<String>) -> String {
    let mut encode_mapping: HashMap<i32, String> = HashMap::new();
    let mut decode_mapping: HashMap<FrozenSet<char>, i32> = HashMap::new();

    let easy_numbers: HashMap<i32, i32> =
        [(1, 2), (7, 3), (4, 4), (8, 7)].iter().cloned().collect();

    for (number, segment_amount) in easy_numbers.iter() {
        let encoded_number = entry_in
            .iter()
            .filter(|entry| entry.len() == *segment_amount as usize)
            .next()
            .unwrap()
            .clone();
        encode_mapping.insert(*number, encoded_number.clone());
        decode_mapping.insert(encoded_number.chars().collect(), *number);
    }

    entry_out
        .iter()
        .map(|entry| {
            decode_mapping
                .get(&FrozenSet::from_iter(entry.chars()))
                .unwrap_or(&-1)
                .to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn decode(entry_in: &Vec<String>, entry_out: &Vec<String>) -> String {
    let mut encode_mapping: HashMap<i32, FrozenSet<char>> = HashMap::new();
    let mut decode_mapping: HashMap<FrozenSet<char>, i32> = HashMap::new();

    let easy_numbers: HashMap<i32, i32> =
        [(1, 2), (7, 3), (4, 4), (8, 7)].iter().cloned().collect();

    for (number, segment_amount) in easy_numbers.iter() {
        let encoded_number = entry_in
            .iter()
            .filter(|entry| entry.len() == *segment_amount as usize)
            .next()
            .unwrap()
            .clone();
        encode_mapping.insert(
            *number,
            FrozenSet::from_iter(encoded_number.clone().chars()),
        );
        decode_mapping.insert(encoded_number.chars().collect(), *number);
    }

    type WordSet = HashSet<FrozenSet<char>>;

    let mut possible_0_6_9: WordSet = entry_in
        .iter()
        .filter(|entry| entry.len() == 6)
        .map(|entry| entry.chars())
        .map(|chars| chars.collect())
        .collect();

    let mut possible_2_3_5: WordSet = entry_in
        .iter()
        .filter(|entry| entry.len() == 5)
        .map(|entry| entry.chars())
        .map(|chars| chars.collect())
        .collect();

    //  resolving  6
    let seven = encode_mapping.get(&7).unwrap();
    let encoded_6 =  identify_encoding(&possible_0_6_9, &|entry| !seven.is_subset(entry));
    encode_mapping.insert(6, encoded_6.clone());
    decode_mapping.insert(encoded_6.clone(), 6);

    possible_0_6_9.remove(&encoded_6);

    // resolving 0
    let four = encode_mapping.get(&4).unwrap();
    let encoded_0 =  identify_encoding(&possible_0_6_9, &|entry|  !four.is_subset(entry));
    encode_mapping.insert(0, encoded_0.clone());
    decode_mapping.insert(encoded_0, 0);

    possible_0_6_9.remove(&encoded_6);

    // resolving 9
    let four = encode_mapping.get(&4).unwrap();
    let encoded_9 =  identify_encoding(&possible_0_6_9, &|entry|  four.is_subset(entry));
    encode_mapping.insert(9, encoded_9.clone());
    decode_mapping.insert(encoded_9, 9);


    // resolving 2
    let nine = encode_mapping.get(&9).unwrap();
    let encoded_2 =  identify_encoding(&possible_2_3_5, &|entry|  !entry.is_subset(nine));
    encode_mapping.insert(2, encoded_2.clone());
    decode_mapping.insert(encoded_2.clone(), 2);

    possible_2_3_5.remove(&encoded_2);

    // resolving 3
    let six = encode_mapping.get(&6).unwrap();
    let encoded_3 =  identify_encoding(&possible_2_3_5, &|entry|  !entry.is_subset(six));
    encode_mapping.insert(3, encoded_3.clone());
    decode_mapping.insert(encoded_3.clone(), 3);

    possible_2_3_5.remove(&encoded_3);


    // resolving 5
    let six = encode_mapping.get(&6).unwrap();
    let encoded_5 =  identify_encoding(&possible_2_3_5, &|entry|  entry.is_subset(six));
    encode_mapping.insert(5, encoded_5.clone());
    decode_mapping.insert(encoded_5.clone(), 5);

    entry_out
        .iter()
        .map(|entry| {
            decode_mapping
                .get(&FrozenSet::from_iter(entry.chars()))
                .unwrap_or(&-1)
                .to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn identify_encoding(
    possibilities: &HashSet<FrozenSet<char>>,
    filter: &dyn Fn(&&FrozenSet<char>) -> bool,
) -> FrozenSet<char> {
    possibilities.iter().filter(filter).next().unwrap().clone()
}
