use ahash::HashMap;

const INPUT_FILE: &str = "input/01.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        for (ind, token) in line.split_ascii_whitespace().enumerate() {
            let parsed: i64 = token.parse().unwrap();
            if ind == 0 {
                left.push(parsed);
            } else if ind == 1 {
                right.push(parsed);
            } else {
                panic!("Bad ind {ind}");
            }
        }
    }

    left.sort();
    right.sort();

    let mut total = 0;

    for (a, b) in left.into_iter().zip(right) {
        total += (a - b).abs();
    }

    total
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> i64 {
    let mut left: HashMap<i64, i64> = HashMap::default();
    let mut right: HashMap<i64, i64> = HashMap::default();

    for line in input.lines() {
        for (ind, token) in line.split_ascii_whitespace().enumerate() {
            let parsed: i64 = token.parse().unwrap();
            if ind == 0 {
                *left.entry(parsed).or_default() += 1;
            } else if ind == 1 {
                *right.entry(parsed).or_default() += 1;
            } else {
                panic!("Bad ind {ind}");
            }
        }
    }

    let mut total = 0;

    for (val, left_count) in left {
        let right_count = right.get(&val).copied().unwrap_or_default();
        total += val * left_count * right_count;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(a_with_input(sample_input), 11);
    }

    #[test]
    fn sample_b() {
        let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(b_with_input(sample_input), 31);
    }
}
