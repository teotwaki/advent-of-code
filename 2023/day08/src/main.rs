use std::{
    collections::HashMap,
    io::{self, Read},
};

use num::integer::lcm;
use winnow::{
    ascii::alphanumeric1,
    combinator::{delimited, separated_pair},
    PResult, Parser,
};

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

impl From<char> for Side {
    fn from(value: char) -> Self {
        match value {
            'L' => Side::Left,
            'R' => Side::Right,
            _ => unreachable!(),
        }
    }
}

struct Map<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
    instructions: Vec<Side>,
}

impl Map<'_> {
    fn traverse(&self, start: &str, is_end: fn(&str) -> bool) -> usize {
        let mut pos = start;
        let mut i = 0;

        loop {
            for inst in &self.instructions {
                pos = match inst {
                    Side::Left => self.map[pos].0,
                    Side::Right => self.map[pos].1,
                };

                i += 1;

                if is_end(pos) {
                    return i;
                }
            }
        }
    }
}

impl<'a> From<&'a str> for Map<'a> {
    fn from(s: &'a str) -> Map<'a> {
        let parts: Vec<_> = s.split("\n\n").collect();

        let instructions = parts[0].chars().map(Side::from).collect();
        let map = parts[1]
            .lines()
            .map(|mut l| line(&mut l).unwrap())
            .collect();

        Map { instructions, map }
    }
}

fn coords<'i>(s: &mut &'i str) -> PResult<&'i str> {
    alphanumeric1.parse_next(s)
}

fn dst<'i>(s: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    delimited('(', separated_pair(coords, ", ", coords), ')').parse_next(s)
}

fn line<'i>(s: &mut &'i str) -> PResult<(&'i str, (&'i str, &'i str))> {
    separated_pair(coords, " = ", dst).parse_next(s)
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Couldn't read stdin");

    input
}

fn main() {
    let input = read_input();
    let map = Map::from(input.as_str());

    let steps = map.traverse("AAA", |s| s == "ZZZ");
    println!("Steps: {}", steps);

    let ghost_steps = map
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| map.traverse(k, |s| s.ends_with('Z')))
        .fold(1, lcm);

    println!("Ghost steps: {}", ghost_steps);
}
