use std::{
    collections::HashSet,
    io::{self, BufRead},
    str::Chars,
};

fn has_unique_elements(chars: Chars) -> bool {
    let mut uniq = HashSet::new();
    chars.into_iter().all(move |x| uniq.insert(x))
}

fn find_marker(s: &str, length: usize) -> usize {
    for i in 0..(s.len() - length) {
        if has_unique_elements(s[i..(i + length)].chars()) {
            return i + length;
        }
    }
    unreachable!()
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("Failed to read stdin"))
        .collect();

    println!("Packet marker at position: {}", find_marker(&lines[0], 4));
    println!("Message marker at position: {}", find_marker(&lines[0], 14));
}
