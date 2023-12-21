use std::io::{self, Read};

#[derive(Debug)]
struct Pattern<'a> {
    rows: Vec<&'a str>,
}

impl<'a> From<&'a str> for Pattern<'a> {
    fn from(s: &'a str) -> Self {
        let rows = s.lines().collect();

        Pattern { rows }
    }
}

impl Pattern<'_> {
    fn horizontal_numbers(&self) -> Vec<u32> {
        self.rows
            .iter()
            .map(|row| {
                row.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '#' { Some(1 << i) } else { None })
                    .sum()
            })
            .collect()
    }

    fn vertical_numbers(&self) -> Vec<u32> {
        (0..self.rows[0].len())
            .map(|i| {
                self.rows
                    .iter()
                    .enumerate()
                    .map(|(j, row)| (j, row.chars().nth(i)))
                    .filter_map(|(j, c)| if c == Some('#') { Some(1 << j) } else { None })
                    .sum()
            })
            .collect()
    }

    fn summarize(&self) -> (usize, usize) {
        let (v1, v2) = find_reflection(&self.vertical_numbers());
        let (h1, h2) = find_reflection(&self.horizontal_numbers());

        (v1 + h1 * 100, v2 + h2 * 100)
    }
}

#[inline]
fn bit_diff(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

fn find_reflection(values: &[u32]) -> (usize, usize) {
    values
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| {
            if bit_diff(w[0], w[1]) <= 1 {
                Some(i)
            } else {
                None
            }
        })
        .fold((0, 0), |acc, pos| {
            let mut min = pos;
            let mut max = pos + 1;
            let mut diff = 0;

            loop {
                diff += bit_diff(values[min], values[max]);

                if diff > 1 {
                    return acc;
                }

                if min > 0 && max < values.len() - 1 {
                    min -= 1;
                    max += 1;
                } else {
                    break;
                }
            }

            match diff {
                0 => (acc.0 + pos + 1, acc.1),
                1 => (acc.0, acc.1 + pos + 1),
                _ => acc,
            }
        })
}

fn main() {
    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s).unwrap();
    let patterns: Vec<_> = s.split("\n\n").map(Pattern::from).collect();

    println!(
        "Summary: {:?}",
        patterns
            .iter()
            .map(Pattern::summarize)
            .fold((0, 0), |acc, x| { (acc.0 + x.0, acc.1 + x.1) })
    );
}
