use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Couldn't read stdin");

    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    sums.sort_unstable();

    println!(
        "Top 1 most calories: {}",
        sums.iter().max().expect("Couldn't find max")
    );

    println!(
        "Top 3 most calories sum: {}",
        sums.iter().rev().take(3).sum::<i32>()
    );
}
