use ahash::HashMap;
use ahash::HashSet;

const INPUT_FILE: &str = "input/08.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let ParseResult { antennae_by_freq, bounds } = parse(input);

    let mut confluence_points = HashSet::default();

    for freq_set in antennae_by_freq.values() {
        for i in 0..freq_set.len() {
            let a = freq_set[i];
            for j in 0..freq_set.len() {
                if j == i {
                    continue;
                }

                let b = freq_set[j];

                let after = b.beyond(a);
                if bounds.contains(after) {
                    confluence_points.insert(after);
                }
            }
        }
    }

    confluence_points.len()
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let ParseResult { antennae_by_freq, bounds } = parse(input);

    let mut confluence_points = HashSet::default();

    for freq_set in antennae_by_freq.values() {
        for i in 0..freq_set.len() {
            let a = freq_set[i];
            for j in 0..freq_set.len() {
                if j == i {
                    continue;
                }

                let mut prev = a;
                let mut next = freq_set[j];

                while bounds.contains(next) {
                    confluence_points.insert(next);

                    let next_next = next.beyond(prev);
                    prev = next;
                    next = next_next;
                }
            }
        }
    }

    confluence_points.len()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    /// Returns the point C where (C - self) == (self - other)
    fn beyond(&self, other: Pos) -> Pos {
        Pos {
            x: self.x * 2 - other.x,
            y: self.y * 2 - other.y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Bounds {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Bounds {
    fn contains(&self, pos: Pos) -> bool {
        pos.x >= self.xmin && pos.x <= self.xmax && pos.y >= self.ymin && pos.y <= self.ymax
    }
}

#[derive(Debug)]
struct ParseResult {
    antennae_by_freq: HashMap<char, Vec<Pos>>,
    bounds: Bounds,
}

fn parse(input: &str) -> ParseResult {
    let mut out: HashMap<char, Vec<Pos>> = HashMap::default();

    let mut xmax = 0;
    let mut ymax = 0;

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c != '.' {
                out.entry(c).or_default().push(Pos { x, y });
            }
            xmax = x.max(xmax);
        }
        ymax = y;
    }

    ParseResult {
        antennae_by_freq: out,
        bounds: Bounds {
            xmin: 0,
            ymin: 0,
            xmax,
            ymax,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(a_with_input(input), 14);
    }

    #[test]
    fn sample_b() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(b_with_input(input), 34);
    }
}
