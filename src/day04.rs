const INPUT_FILE: &str = "input/04.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let Grid { rows, height, width } = parse(input);

    let mut count = 0;

    for y in 0 .. height as i64 {
        for x in 0 .. width as i64 {
            if rows[y as usize][x as usize] != 'X' {
                continue;
            }

            for dir in [DirA::U, DirA::D, DirA::L, DirA::R, DirA::UR, DirA::UL, DirA::DR, DirA::DL] {
                if is_match_a(&rows, x, y, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

struct Grid {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
enum DirA {
    U, UR, UL, R, L, D, DR, DL,
}

fn is_match_a(rows: &[Vec<char>], x: i64, y: i64, dir: DirA) -> bool {
    let Vel { dx, dy } = make_vel(dir);

    ['X', 'M', 'A', 'S'].into_iter()
        .enumerate()
        .all(|(i, char_to_match)| {
            let x = x + (i as i64) * dx;
            let y = y + (i as i64) * dy;

            if y < 0 || (y as usize) >= rows.len() {
                return false;
            }

            let y = y as usize;

            if x < 0 || (x as usize) >= rows[y].len() {
                return false;
            }

            let x = x as usize;

            rows[y][x] == char_to_match
        })
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
struct Vel {
    dx: i64, dy: i64,
}

fn make_vel(dir: DirA) -> Vel {
    match dir {
        DirA::U => Vel { dx: 0, dy: -1 },
        DirA::UR => Vel { dx: 1, dy: -1 },
        DirA::UL => Vel { dx: -1, dy: -1 },
        DirA::R => Vel { dx: 1, dy: 0},
        DirA::L => Vel{dx: -1, dy: 0},
        DirA::D => Vel{dx: 0, dy: 1},
        DirA::DR => Vel {dx: 1, dy: 1},
        DirA::DL => Vel { dx: -1, dy: 1 },
    }
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let Grid { rows, height, width } = parse(input);

    let mut count = 0;

    for y in 1 .. height - 1 {
        for x in 1 .. width -1 {
            if rows[y][x] != 'A' {
                continue;
            }

            for dir in [DirB::D, DirB::U, DirB::L, DirB::R] {
                if is_match_b(&rows, x, y, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_match_b(rows: &[Vec<char>], x: usize, y: usize, dir_b: DirB) -> bool {
    if rows[y][x] != 'A' {
        return false;
    }

    match dir_b {
        DirB::L => rows[y-1][x-1] == 'M' && rows[y+1][x-1] == 'M' && rows[y-1][x+1] == 'S' && rows[y+1][x+1] == 'S',
        DirB::R => rows[y-1][x-1] == 'S' && rows[y+1][x-1] == 'S' && rows[y-1][x+1] == 'M' && rows[y+1][x+1] == 'M',
        DirB::U => rows[y-1][x-1] == 'M' && rows[y+1][x-1] == 'S' && rows[y-1][x+1] == 'M' && rows[y+1][x+1] == 'S',
        DirB::D => rows[y-1][x-1] == 'S' && rows[y+1][x-1] == 'M' && rows[y-1][x+1] == 'S' && rows[y+1][x+1] == 'M',
    }
}

/// Indicates the direction the two M's will be, relative to the A
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
enum DirB {
    L,
    R,
    U,
    D,
}

fn parse(input: &str) -> Grid {
    let rows: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = rows.len();
    let width = rows[0].len();

    Grid {
        rows, height, width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(a_with_input(input), 18);
    }

    #[test]
    fn sample_b() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(b_with_input(input), 9);
    }
}
