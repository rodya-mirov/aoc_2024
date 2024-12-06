use regex::Regex;

const INPUT_FILE: &str = "input/03.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> u64 {
    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();

    let mut total = 0;
    for (_, [num_a, num_b]) in re.captures_iter(input).map(|c| c.extract()) {
        let num_a: u64 = num_a.parse().unwrap();
        let num_b: u64 = num_b.parse().unwrap();

        total += num_a * num_b;
    }
    total
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> u64 {
    let re = Regex::new("((do)\\(\\)()())|((don't)\\(\\)()())|((mul)\\(([0-9]+),([0-9]+)\\))").unwrap();

    let mut enabled = true;
    let mut total = 0;
    for (_, [_, tag, num_a, num_b]) in re.captures_iter(input).map(|c| c.extract()) {
        match tag {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                if enabled {
                    let num_a: u64 = num_a.parse().unwrap();
                    let num_b: u64 = num_b.parse().unwrap();

                    total += num_a * num_b;
                }
            }
            other => panic!("Bad tag {other} (probably unreachable)")
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(a_with_input(input), 161);
    }

    #[test]
    fn sample_b() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(b_with_input(input), 48);
    }
}
