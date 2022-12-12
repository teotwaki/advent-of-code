use std::io::{self, BufRead};

fn main() {
    let vecs = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Expected input line"))
        .map(|l| {
            l.split(',')
                .map(|x| {
                    x.split('-')
                        .map(|x| x.parse::<i32>().expect("Expected integer"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<_>>();

    let fully_contained_count = vecs
        .iter()
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count();

    let partial_overlap_count = vecs
        .iter()
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1])
                || (pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1])
                || (pair[1][1] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count();

    println!(
        "Assignments fully contained in pair: {}",
        fully_contained_count
    );

    println!(
        "Assignments with partial overlap: {}",
        partial_overlap_count
    );
}
