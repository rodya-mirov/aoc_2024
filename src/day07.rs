use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

const INPUT_FILE: &str = "input/07.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> u64 {
    let mut out = 0;
    for line in parse(input) {
        if can_be_solved_a(&line) {
            out += line.goal;
        }
    }
    out
}

fn can_be_solved_a(line: &Line) -> bool {
    fn can_be_solved_running(goal: u64, running: u64, nums: &[u64]) -> bool {
        if nums.len() == 0 {
            goal == running
        } else {
            // try each op
            let next = running.checked_add(nums[0]).unwrap();
            if can_be_solved_running(goal, next, &nums[1..]) {
                return true;
            }
            let next = running.checked_mul(nums[0]).unwrap();
            if can_be_solved_running(goal, next, &nums[1..]) {
                return true;
            }

            false
        }
    }

    can_be_solved_running(line.goal, 0, &line.nums)
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> u64 {
    let mut out = 0;
    for line in parse(input) {
        if can_be_solved_b(&line) {
            out += line.goal;
        }
    }
    out
}

fn can_be_solved_b(line: &Line) -> bool {
    fn can_be_solved_running(goal: u64, running: u64, nums: &[u64]) -> bool {
        if nums.len() == 0 {
            goal == running
        } else {
            // try each op
            let next = running.checked_add(nums[0]).unwrap();
            if can_be_solved_running(goal, next, &nums[1..]) {
                return true;
            }
            let next = running.checked_mul(nums[0]).unwrap();
            if can_be_solved_running(goal, next, &nums[1..]) {
                return true;
            }
            let next = concat(running, nums[0]);
            if can_be_solved_running(goal, next, &nums[1..]) {
                return true;
            }

            false
        }
    }

    can_be_solved_running(line.goal, 0, &line.nums)
}

fn concat(mut a: u64, b: u64) -> u64 {
    let mut pow = 1;
    while pow <= b {
        pow *= 10;
        a = a.checked_mul(10).unwrap();
    }
    a.checked_add(b).unwrap()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Line {
    goal: u64,
    nums: Vec<u64>,
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|line| parse_line(line).unwrap().1).collect()
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    fn parse_num(s: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(s)
    }

    let (input, goal) = parse_num(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, nums) = many1(map(tuple((space1, parse_num)), |(_, num)| num))(input).unwrap();

    assert_eq!(input, "");

    Ok(("", Line { goal, nums }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse_line("190: 10 19").unwrap().1,
            Line {
                goal: 190,
                nums: vec![10, 19]
            }
        );
    }

    #[test]
    fn sample_a() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(a_with_input(input), 3749);
    }

    #[test]
    fn sample_b() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(b_with_input(input), 11387);
    }
}
