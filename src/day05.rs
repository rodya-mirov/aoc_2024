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
            assert!(numbering.len() > 0, "Must be nonempty");
            assert_eq!(numbering.len() % 2, 1, "Must be odd length");

            total += numbering[numbering.len() / 2];
        }
    }

    total
}

fn is_list_compatible(orderings: &[(u32, u32)], numbering: &[u32]) -> bool {
    for i in 0..numbering.len() - 1 {
        for j in i + 1..numbering.len() {
            if !is_pair_compatible(numbering[i], numbering[j], orderings) {
                return false;
            }
        }
    }

    true
}

fn is_pair_compatible(a: u32, b: u32, orderings: &[(u32, u32)]) -> bool {
    orderings.iter().copied().all(|pair| pair != (b, a))
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(_input: &str) -> String {
    unimplemented!()
}

struct Parsed {
    orderings: Vec<(u32, u32)>,
    numberings: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();

    let mut orderings = Vec::new();
    let pair_re = Regex::new("^([0-9]+)\\|([0-9]+)$").unwrap();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (_, [num_a, num_b]) = pair_re.captures(line).unwrap().extract();
        let num_a: u32 = num_a.parse().unwrap();
        let num_b: u32 = num_b.parse().unwrap();

        orderings.push((num_a, num_b));
    }

    let mut numberings = Vec::new();

    while let Some(line) = lines.next() {
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
}
