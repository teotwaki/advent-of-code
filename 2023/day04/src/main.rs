use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Card {
    copies: u32,
    winning_numbers: Vec<u8>,
    my_numbers: Vec<u8>,
}

impl Card {
    fn points(&self) -> u16 {
        match self.matches() {
            0 => 0,
            n => 2_u16.pow((n - 1) as u32),
        }
    }

    fn matches(&self) -> u8 {
        self.my_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u8
    }

    fn increment(&mut self, copies: u32) {
        self.copies += copies;
    }
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Couldn't read stdin");

    input
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| &l[l.find(": ").unwrap() + 2..])
        .map(|l| {
            l.split(" | ")
                .map(|s| {
                    s.split(' ')
                        .filter_map(|d| d.parse::<u8>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|v| Card {
            copies: 1,
            winning_numbers: v[0].clone(),
            my_numbers: v[1].clone(),
        })
        .collect()
}

fn main() {
    let input = read_input();
    let mut cards = parse(&input);

    let sum: u16 = cards.iter().map(|c| c.points()).sum();

    for i in 0..cards.len() {
        let matches = cards[i].matches();
        let copies = cards[i].copies;

        for j in (1..=matches).rev() {
            let idx = i + j as usize;
            cards[idx].increment(copies);
        }
    }

    let card_sum: u32 = cards.iter().map(|c| c.copies as u32).sum();

    println!("Sum of all scratch card points: {}", sum);
    println!("Count of all scratch cards: {}", card_sum);
}
