use ahash::HashSet;

const INPUT_FILE: &str = "input/06.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let Parsed {
        cells,
        start_pos,
        start_dir,
    } = parse(input);

    let tr = traverse(&cells, start_pos, start_dir);
    assert!(!tr.is_loop);

    tr.cells_seen.len()
}

struct TraverseResult {
    is_loop: bool,
    cells_seen: HashSet<Pos>,
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let Parsed {
        mut cells,
        start_pos,
        start_dir,
    } = parse(input);

    let mut num_loops = 0;

    // find the spots the guard actually walks across; no sense putting an obstruction anywhere else
    let allowed_spots = traverse(&cells, start_pos, start_dir).cells_seen;

    for Pos { y, x } in allowed_spots {
        // for each conceivable spot, simply fill it in, see if you get a loop, and count the result
        if (cells[y][x] == Square::Empty) && (Pos { x, y } != start_pos) {
            cells[y][x] = Square::Filled;
            if traverse(&cells, start_pos, start_dir).is_loop {
                num_loops += 1;
            }
            cells[y][x] = Square::Empty;
        }
    }

    num_loops
}

fn traverse(cells: &[Vec<Square>], start_pos: Pos, start_dir: Dir) -> TraverseResult {
    let mut dir = start_dir;
    let mut pos = start_pos;

    let mut seen: HashSet<(Pos, Dir)> = HashSet::default();

    loop {
        if !seen.insert((pos, dir)) {
            let seen_pos: HashSet<Pos> = seen.into_iter().map(|(pos, _)| pos).collect();
            return TraverseResult {
                cells_seen: seen_pos,
                is_loop: true,
            };
        }

        let next_pos = pos.advance(dir, cells);

        if next_pos.is_none() {
            let seen_pos: HashSet<Pos> = seen.into_iter().map(|(pos, _)| pos).collect();
            return TraverseResult {
                cells_seen: seen_pos,
                is_loop: false,
            };
        }

        let next_pos = next_pos.unwrap();

        let cell = cells[next_pos.y][next_pos.x];

        match cell {
            Square::Filled => {
                dir = dir.right();
            }
            Square::Empty => {
                pos = next_pos;
            }
        }
    }
}

fn parse(input: &str) -> Parsed {
    let mut cells = Vec::new();
    let mut start_pos = None;
    let mut start_dir = None;

    for (row_idx, row_line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (col_idx, cell_char) in row_line.chars().enumerate() {
            let square = match cell_char {
                '.' => Square::Empty,
                '#' => Square::Filled,
                '^' => {
                    if start_dir.is_some() || start_pos.is_some() {
                        panic!("Already saw a start pos!");
                    }

                    start_dir = Some(Dir::U);
                    start_pos = Some(Pos { x: col_idx, y: row_idx });

                    Square::Empty
                }
                other => {
                    panic!("Unexpected cell char '{other}'")
                }
            };
            row.push(square);
        }

        cells.push(row);
    }

    Parsed {
        cells,
        start_pos: start_pos.unwrap(),
        start_dir: start_dir.unwrap(),
    }
}

struct Parsed {
    cells: Vec<Vec<Square>>,
    start_pos: Pos,
    start_dir: Dir,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn advance(self, dir: Dir, cells: &[Vec<Square>]) -> Option<Pos> {
        let ymax = cells.len();
        let xmax = cells[0].len();

        match dir {
            Dir::U => self.y.checked_sub(1).map(|y| Pos { x: self.x, y }),
            Dir::R => self.x.checked_add(1).filter(|&x| x < xmax).map(|x| Pos { x, y: self.y }),
            Dir::D => self.y.checked_add(1).filter(|&y| y < ymax).map(|y| Pos { x: self.x, y }),
            Dir::L => self.x.checked_sub(1).map(|x| Pos { x, y: self.y }),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Square {
    Filled,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn right(self) -> Self {
        match self {
            Dir::U => Dir::R,
            Dir::R => Dir::D,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(a_with_input(input), 41);
    }

    #[test]
    fn sample_b() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(b_with_input(input), 6);
    }
}
