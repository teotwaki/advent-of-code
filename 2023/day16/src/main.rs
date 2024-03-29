use std::io::{self, Read};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Tile {
    content: Content,
    energized_from: Vec<Direction>,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Self {
            content: Content::from(c),
            energized_from: vec![],
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Content {
    Empty,
    MirrorSWNE,
    MirrorNWSE,
    SplitterHorizontal,
    SplitterVertical,
}

impl From<char> for Content {
    fn from(c: char) -> Self {
        use Content::*;

        match c {
            '/' => MirrorSWNE,
            '\\' => MirrorNWSE,
            '-' => SplitterHorizontal,
            '|' => SplitterVertical,
            _ => Empty,
        }
    }
}

struct Layout {
    rows: Vec<Vec<Tile>>,
}

impl From<&str> for Layout {
    fn from(s: &str) -> Self {
        let rows: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();

        Layout { rows }
    }
}

impl Layout {
    fn count_energized(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| !tile.energized_from.is_empty())
                    .count()
            })
            .sum()
    }

    fn reset_energized(&mut self) {
        self.rows
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|tile| tile.energized_from.clear()));
    }

    fn entrypoints(&self) -> Vec<((usize, usize), Direction)> {
        use Direction::*;
        let mut v = vec![];

        for i in 0..self.rows.len() {
            v.push(((0, i), Right));
            v.push(((self.rows.len() - 1, i), Left));
        }

        for i in 0..self.rows[0].len() {
            v.push(((i, 0), Down));
            v.push(((i, self.rows.len() - 1), Up));
        }

        v
    }

    fn traverse(&mut self, entrypoint: ((usize, usize), Direction)) {
        let mut stack = vec![entrypoint];

        while let Some(((x, y), direction)) = stack.pop() {
            if self.rows[y][x].energized_from.contains(&direction) {
                continue;
            } else {
                self.rows[y][x].energized_from.push(direction);
            }

            use Content::*;
            use Direction::*;

            match (self.rows[y][x].content, direction) {
                (Empty | SplitterHorizontal, Right) => {
                    if x < (self.rows[y].len() - 1) {
                        stack.push(((x + 1, y), Right))
                    }
                }
                (Empty | SplitterHorizontal, Left) => {
                    if x > 0 {
                        stack.push(((x - 1, y), Left))
                    }
                }
                (Empty | SplitterVertical, Up) => {
                    if y > 0 {
                        stack.push(((x, y - 1), Up))
                    }
                }
                (Empty | SplitterVertical, Down) => {
                    if y < (self.rows.len() - 1) {
                        stack.push(((x, y + 1), Down))
                    }
                }
                (SplitterVertical, Right | Left) => {
                    if y > 0 {
                        stack.push(((x, y - 1), Up))
                    }
                    if y < (self.rows.len() - 1) {
                        stack.push(((x, y + 1), Down))
                    }
                }
                (SplitterHorizontal, Up | Down) => {
                    if x > 0 {
                        stack.push(((x - 1, y), Left))
                    }
                    if x < (self.rows[y].len() - 1) {
                        stack.push(((x + 1, y), Right))
                    }
                }
                (MirrorSWNE, Right) | (MirrorNWSE, Left) => {
                    if y > 0 {
                        stack.push(((x, y - 1), Up))
                    }
                }
                (MirrorSWNE, Down) | (MirrorNWSE, Up) => {
                    if x > 0 {
                        stack.push(((x - 1, y), Left))
                    }
                }
                (MirrorSWNE, Up) | (MirrorNWSE, Down) => {
                    if x < (self.rows[y].len() - 1) {
                        stack.push(((x + 1, y), Right))
                    }
                }
                (MirrorSWNE, Left) | (MirrorNWSE, Right) => {
                    if y < (self.rows.len() - 1) {
                        stack.push(((x, y + 1), Down))
                    }
                }
            }
        }
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s).unwrap();

    let mut layout = Layout::from(s.as_str());
    layout.traverse(((0, 0), Direction::Right));

    let energized = layout.count_energized();

    println!("Initial number of energized tiles: {energized}");

    layout.reset_energized();

    let entrypoints = layout.entrypoints();

    let max_energized = entrypoints
        .into_iter()
        .map(|e| {
            layout.reset_energized();
            layout.traverse(e);
            layout.count_energized()
        })
        .max()
        .unwrap();

    println!("Highest number of energized tiles: {max_energized}");
}
