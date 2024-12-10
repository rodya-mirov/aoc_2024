use ahash::{HashMap, HashSet};
use regex::Regex;

const INPUT_FILE: &str = "input/05.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> u32 {
    let parsed = parse(input);

    let mut total = 0;

    for numbering in parsed.numberings.iter() {
        if is_list_compatible(&parsed.orderings, numbering.as_slice()) {
            assert!(!numbering.is_empty(), "Must be nonempty");
            assert_eq!(numbering.len() % 2, 1, "Must be odd length");

            total += numbering[numbering.len() / 2];
        }
    }

    total
}

fn is_list_compatible(orderings: &HashMap<u32, HashSet<u32>>, numbering: &[u32]) -> bool {
    for i in 0..numbering.len() - 1 {
        for j in i + 1..numbering.len() {
            if orderings.get(&numbering[j]).map(|set| set.contains(&numbering[i])).unwrap_or(false) {
                return false;
            }
        }
    }

    true
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> u32 {
    let mut parsed = parse(input);

    let mut total = 0;

    for numbering in parsed.numberings.iter_mut() {
        if !is_list_compatible(&parsed.orderings, numbering.as_slice()) {
            sort_numbering(&parsed.orderings, numbering);
            assert!(!numbering.is_empty(), "Must be nonempty");
            assert_eq!(numbering.len() % 2, 1, "Must be odd length");

            total += numbering[numbering.len() / 2];
        }
    }

    total
}

fn sort_numbering(orderings: &HashMap<u32, HashSet<u32>>, numbering: &mut [u32]) {
    numbering.sort_by(|a, b| {
        if orderings.get(a).map(|s| s.contains(b)).unwrap_or(false) {
            std::cmp::Ordering::Less
        } else if orderings.get(b).map(|s| s.contains(a)).unwrap_or(false) {
            std::cmp::Ordering::Greater
        } else if a == b {
            std::cmp::Ordering::Equal
        } else {
            panic!("I assumed all edges would be supplied, but {a} vs {b} is not")
        }
    })
}

struct Parsed {
    // map x -> (things > x)
    orderings: HashMap<u32, HashSet<u32>>,
    numberings: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();

    let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::default();
    let pair_re = Regex::new("^([0-9]+)\\|([0-9]+)$").unwrap();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (_, [num_a, num_b]) = pair_re.captures(line).unwrap().extract();
        let num_a: u32 = num_a.parse().unwrap();
        let num_b: u32 = num_b.parse().unwrap();

        orderings.entry(num_a).or_default().insert(num_b);
    }

    let mut numberings = Vec::new();

    for line in lines {
        let numbering: Vec<u32> = line.split(",").map(|token| token.parse::<u32>().unwrap()).collect();
        numberings.push(numbering);
    }

    Parsed { orderings, numberings }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(a_with_input(input), 143);
    }

    #[test]
    fn sample_b() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(b_with_input(input), 123);
    }
}
