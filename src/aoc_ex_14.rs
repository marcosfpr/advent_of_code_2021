use std::collections::HashMap;

#[path = "aoc_utils.rs"]
mod aoc_utils;

struct Rule {
    pattern: String,
    result: String,
}

pub fn resolve_part_1(filename: &'static str, steps: i32) -> u64 {
    // Read the input file  and parse it into a vector of strings
    let polymers: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let mut polymers_iter = polymers.iter();

    let template_string = polymers_iter.next().unwrap().clone();

    // skip next line
    polymers_iter.next();

    //next lines: pair rules
    let rules: Vec<Rule> = polymers_iter
        .map(|r| {
            let mut rule = r.split(" -> ");
            Rule {
                pattern: rule.next().unwrap().trim().to_string(),
                result: rule.next().unwrap().trim().to_string(),
            }
        })
        .collect();

    // Apply the rules to the template
    let final_string = execute_automata(template_string, &rules, steps);

    let mut counts: HashMap<char, u64> = HashMap::new();

    for c in final_string.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", counts);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

// slow
fn execute_automata(template: String, rules: &Vec<Rule>, steps: i32) -> String {
    let mut template_string = template;

    for _ in 0..steps {
        let end = template_string.len();
        // automata execution
        for j in (1..end).rev() {
            let i = j - 1;
            let slice = &template_string.clone()[..];

            let substr = &slice[i..(j + 1)]; // substring

            rules.iter().filter(|r| r.pattern == substr).for_each(|r| {
                template_string = template_string[0..(i + 1)].to_string()
                    + &r.result
                    + &template_string[(i + 1)..];
            });
        }

        println!("polymer size: {:?}", template_string.len());
    }
    template_string
}

// fast
pub fn resolve_part_2(filename: &'static str, steps: usize) -> usize {
    // Read the input file  and parse it into a vector of strings
    let polymers: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");

    let mut polymers_iter = polymers.iter();

    let template: Vec<char> = polymers_iter.next().unwrap().clone().chars().collect();

    // skip next line
    polymers_iter.next();

    //next lines: pair rules
    let rules = polymers_iter
        .map(|r| {
            let (pattern, addition) = r.split_once(" -> ").unwrap();
            let mut pattern = pattern.chars();

            let p1 = pattern.next().unwrap();
            let p2 = pattern.next().unwrap();

            let addition = addition.chars().next().unwrap();

            ((p1, p2), addition)
        })
        .collect();

    // memoization
    let mut memo: HashMap<(char, char, usize), HashMap<char, usize>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 1..template.len() {
        let p1 = template[i - 1];
        let p2 = template[i];

        for (c, count) in count(p1, p2, &rules, &mut memo, steps) {
            *counts.entry(c).or_default() += count;
        }

        if i != 1 {
            *counts.entry(p1).or_default() -= 1;
        }
    }

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();

    most_common - least_common
}

fn count(
    p1: char,
    p2: char,
    rules: &HashMap<(char, char), char>,
    memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    iter: usize,
) -> HashMap<char, usize> {
    if iter == 0 {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        return counts;
    }

    if let Some(result) = memo.get(&(p1, p2, iter)) {
        return result.clone();
    }

    if let Some(c) = rules.get(&(p1, p2)) {
        let mut counts = HashMap::new();

        for (c, count) in count(p1, *c, rules, memo, iter - 1) {
            *counts.entry(c).or_default() += count;
        }
        for (c, count) in count(*c, p2, rules, memo, iter - 1) {
            *counts.entry(c).or_default() += count;
        }

        *counts.entry(*c).or_default() -= 1;
        memo.insert((p1, p2, iter), counts.clone());
        counts
    } else {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        counts
    }
}
