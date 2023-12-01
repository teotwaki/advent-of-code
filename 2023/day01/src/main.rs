use std::io::{self, Read};

fn chars_to_number(first: char, second: char) -> u32 {
    first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap()
}

#[cfg(not(feature = "step2"))]
fn clean_input(input: String) -> String {
    input
}

#[cfg(feature = "step2")]
fn clean_input(input: String) -> String {
    input
        .replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Couldn't read stdin");

    input = clean_input(input);

    let sum: u32 = input
        .lines()
        .map(|l| {
            (
                l.chars().find(|c| c.is_numeric()).unwrap(),
                l.chars().rev().find(|c| c.is_numeric()).unwrap(),
            )
        })
        .map(|(c1, c2)| chars_to_number(c1, c2))
        .sum();

    println!("Total sum: {}", sum);
}
