use std::io::{self, BufRead};

fn main() {
    let (first, last) = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|hist| {
            let mut diffs = vec![hist];

            loop {
                diffs.push(
                    diffs
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );

                if diffs.last().unwrap().iter().all(|x| *x == 0) {
                    break;
                }
            }

            (
                diffs
                    .iter()
                    .map(|d| *d.first().unwrap())
                    .rev()
                    .fold(0, |acc, x| x - acc),
                diffs.iter().map(|d| *d.last().unwrap()).sum::<i32>(),
            )
        })
        .fold((0, 0), |(accf, accl), (f, l)| (accf + f, accl + l));

    println!("Sum of all predictions: {}", last);
    println!("Sum of all extrapolations: {}", first);
}
