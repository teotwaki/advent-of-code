use std::io::{self, BufRead};

fn priority(c: char) -> i32 {
    let ascii: i32 = c as i32;

    if ascii >= 'a' as i32 {
        ascii - 'a' as i32 + 1
    } else {
        ascii - 'A' as i32 + 27
    }
}

fn find_common(group: &[String]) -> i32 {
    group
        .iter()
        .map(|l| split_halves(l))
        .map(|(left, right)| {
            for c in left.chars() {
                if right.contains(c) {
                    return priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

fn find_badge(group: &[String]) -> char {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }
    unreachable!()
}

fn split_halves(l: &str) -> (String, String) {
    (
        l[0..(l.len() / 2)].to_string(),
        l[(l.len() / 2)..l.len()].to_string(),
    )
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let (badge_priorities, item_priorities): (Vec<i32>, Vec<i32>) = lines
        .chunks(3)
        .map(|group| {
            let badge_priority = priority(find_badge(group));
            let item_priorities = find_common(group);

            (badge_priority, item_priorities)
        })
        .unzip();

    println!("Step 1: {:?}", item_priorities.iter().sum::<i32>());
    println!("Step 2: {:?}", badge_priorities.iter().sum::<i32>());
}
