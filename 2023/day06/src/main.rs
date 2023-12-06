struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn outcomes(&self) -> Vec<u64> {
        (0..=self.time)
            .map(|speed| (self.time - speed) * speed)
            .collect()
    }
}

fn sample_races() -> Vec<Race> {
    vec![
        Race {
            time: 7,
            distance: 9,
        },
        Race {
            time: 15,
            distance: 40,
        },
        Race {
            time: 30,
            distance: 200,
        },
    ]
}

fn real_races() -> Vec<Race> {
    vec![
        Race {
            time: 44,
            distance: 277,
        },
        Race {
            time: 89,
            distance: 1136,
        },
        Race {
            time: 96,
            distance: 1890,
        },
        Race {
            time: 91,
            distance: 1768,
        },
    ]
}

fn sample_step2_races() -> Vec<Race> {
    vec![Race {
        time: 71530,
        distance: 940200,
    }]
}

fn real_step2_races() -> Vec<Race> {
    vec![Race {
        time: 44899691,
        distance: 277113618901768,
    }]
}

fn main() {
    let races = real_step2_races();

    let product: usize = races
        .iter()
        .map(|r| r.outcomes().iter().filter(|o| **o > r.distance).count())
        .product();

    println!("Product of all ways we can beat the races: {}", product);
}
