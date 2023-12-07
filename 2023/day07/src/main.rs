use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    #[cfg(feature = "step2")]
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    #[cfg(not(feature = "step2"))]
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            #[cfg(feature = "step2")]
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            #[cfg(not(feature = "step2"))]
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Hand {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<_> = s.chars().map(Card::from).collect();

        let mut freq: Vec<_> = cards
            .into_iter()
            .fold(HashMap::<Card, usize>::new(), |mut map, card| {
                *map.entry(card).or_default() += 1;
                map
            })
            .into_iter()
            .collect();

        freq.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            ord => ord,
        });

        #[cfg(feature = "step2")]
        {
            for i in 0..freq.len() {
                if freq[i].0 == Card::Joker {
                    if freq.len() == 1 {
                        break;
                    }

                    let dst = if i == 0 { 1 } else { 0 };

                    freq[dst].1 += freq[i].1;
                    freq.remove(i);
                    break;
                }
            }
        }

        let hand = match freq.len() {
            5 => Hand::HighCard,
            4 => Hand::Pair,
            3 if freq[0].1 == 2 => Hand::TwoPairs,
            3 if freq[0].1 == 3 => Hand::ThreeOfAKind,
            2 if freq[0].1 == 3 => Hand::FullHouse,
            2 => Hand::FourOfAKind,
            1 => Hand::FiveOfAKind,
            _ => unreachable!(),
        };

        Ok(hand)
    }
}

#[derive(Debug)]
struct Round {
    cards: Vec<Card>,
    hand: Hand,
    bid: u16,
}

fn main() {
    let mut rounds: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Round {
            cards: l[..5].chars().map(Card::from).collect(),
            hand: Hand::from_str(&l[..5]).unwrap(),
            bid: l[6..].parse().unwrap(),
        })
        .collect();

    rounds.sort_by(|a, b| match a.hand.cmp(&b.hand) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        ord => ord,
    });

    let sum: usize = rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) * r.bid as usize)
        .sum();

    println!("Total winnings: {}", sum);
}
