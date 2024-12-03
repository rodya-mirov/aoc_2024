const INPUT_FILE: &str = "input/02.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line: Vec<i32> = line.split_ascii_whitespace().map(|tok| tok.parse::<i32>().unwrap()).collect();
            line
        })
        .filter(|line| is_safe(line.as_slice()))
        .count()
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line: Vec<i32> = line.split_ascii_whitespace().map(|tok| tok.parse::<i32>().unwrap()).collect();
            line
        })
        .filter(|line| is_safe_damped(line.as_slice()))
        .count()
}

fn is_safe_damped(line: &[i32]) -> bool {
    if is_safe(line) {
        return true;
    }

    for skipped_index in 0..line.len() {
        // it doesn't SEEM like a really fast algorithm
        // but like, it runs in a third of a millisecond
        let sub_line: Vec<i32> = line
            .iter()
            .copied()
            .enumerate()
            .filter(|(ind, _val)| *ind != skipped_index)
            .map(|(_ind, val)| val)
            .collect();
        if is_safe(sub_line.as_slice()) {
            return true;
        }
    }

    false
}

fn is_safe(line: &[i32]) -> bool {
    let mut any_inc = false;
    let mut any_dec = false;

    for i in 1..line.len() {
        let a = line[i - 1];
        let b = line[i];
        let d = (a - b).abs();

        if d == 0 || d > 3 {
            return false;
        }

        if a < b {
            any_inc = true;
        } else {
            any_dec = true;
        }
    }

    !(any_inc && any_dec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(a_with_input(input), 2);
    }

    #[test]
    fn sample_b() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(b_with_input(input), 4);
    }
}
