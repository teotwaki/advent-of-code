use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    io::{self, BufRead},
};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct SensorReadout {
    sensor: Position,
    nearest_beacon: Position,
}

impl SensorReadout {
    fn new(s: Position, b: Position) -> Self {
        Self {
            sensor: s,
            nearest_beacon: b,
        }
    }
}

#[derive(Debug, PartialEq)]
enum LocationContents {
    Empty,
    Sensor,
    Beacon,
}

impl Display for LocationContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '#',
            Self::Sensor => 'S',
            Self::Beacon => 'B',
        };

        f.write_char(c)
    }
}

#[derive(Debug)]
struct Map {
    columns: HashMap<i32, HashMap<i32, LocationContents>>,
}

impl Map {
    fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    fn insert(&mut self, pos: &Position, contents: LocationContents) {
        self.columns
            .entry(pos.x)
            .or_default()
            .insert(pos.y, contents);
    }

    fn min_max_x(&self) -> (i32, i32) {
        let keys = self.columns.keys();
        let min = keys.clone().min().expect("Couldn't find minimum x value");
        let max = keys.max().expect("Couldn't find maximum x value");

        (*min, *max)
    }

    fn min_max_y(&self) -> (i32, i32) {
        let keys = self.columns.iter().flat_map(|(_, c)| c.keys());
        let min = keys.clone().min().expect("Couldn't find minimum y value");
        let max = keys.max().expect("Couldn't find maximum y value");

        (*min, *max)
    }

    fn get(&self, pos: &Position) -> Option<&LocationContents> {
        self.columns.get(&pos.x).and_then(|c| c.get(&pos.y))
    }

    fn unavailable_locations_in_row(&self, row: i32) -> usize {
        let (min_x, max_x) = self.min_max_x();
        let mut count = 0;

        for x in min_x..=max_x {
            let p = Position { x, y: row };
            if Some(&LocationContents::Empty) == self.get(&p) {
                count += 1;
            }
        }

        count
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self.min_max_x();
        let (min_x, max_x) = (min_x - 3, max_x + 3);
        let (min_y, max_y) = self.min_max_y();
        let (min_y, max_y) = (min_y - 3, max_y + 3);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Position { x, y };
                if let Some(item) = self.get(&p) {
                    f.write_fmt(format_args!("{}", item))?;
                } else {
                    f.write_char('.')?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn extract_data(s: &str) -> SensorReadout {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Sensor at x=(?P<sx>[^,]+), y=(?P<sy>[^:]+): closest beacon is at x=(?P<bx>[^,]+), y=(?P<by>.+)$"
        )
        .expect("Invalid regex");
    }
    RE.captures(s)
        .map(|c| {
            (
                c.name("sx").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("sy").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("bx").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("by").and_then(|m| m.as_str().parse().ok()).unwrap(),
            )
        })
        .map(|(sx, sy, bx, by)| SensorReadout::new(Position::new(sx, sy), Position::new(bx, by)))
        .unwrap()
}

fn distance(left: &Position, right: &Position) -> i32 {
    (left.x.abs_diff(right.x) + left.y.abs_diff(right.y))
        .try_into()
        .unwrap()
}

fn calculate_range(sensor: &Position, beacon: &Position) -> Vec<Position> {
    let delta_x = sensor.x.abs_diff(beacon.x) as i32 * 4;
    let delta_y = sensor.y.abs_diff(beacon.y) as i32 * 4;

    let min_x = sensor.x - delta_x;
    let max_x = sensor.x + delta_x;
    let min_y = sensor.y - delta_y;
    let max_y = sensor.y + delta_y;

    let max_distance = distance(sensor, beacon);

    let mut positions = vec![];
    for x in min_x..max_x {
        for y in min_y..max_y {
            let pos = Position::new(x, y);
            let dist = distance(sensor, &pos);

            if dist <= max_distance {
                positions.push(pos);
            }
        }
    }

    positions
}

fn main() {
    let mut map = Map::new();

    let readouts: Vec<SensorReadout> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Couldn't read from stdin"))
        .map(|l| extract_data(&l))
        .collect();

    println!("Finished parsing file");

    readouts
        .iter()
        .inspect(|r| println!("{:?}", r))
        .flat_map(|r| calculate_range(&r.sensor, &r.nearest_beacon))
        .inspect(|r| println!("{:?}", r))
        .for_each(|pos| map.insert(&pos, LocationContents::Empty));

    println!("Finished calculating ranges for beacons");

    readouts.iter().for_each(|r| {
        map.insert(&r.sensor, LocationContents::Sensor);
        map.insert(&r.nearest_beacon, LocationContents::Beacon);
    });

    println!("Finished inserting sensors and beacons");

    let (_, max_y) = map.min_max_y();

    println!("Max y: {}", max_y);

    if max_y == 26 {
        // small_input
        println!(
            "Unavailable locations in row 10: {}",
            map.unavailable_locations_in_row(10)
        );
    } else {
        println!(
            "Unavailable locations in row 2000000: {}",
            map.unavailable_locations_in_row(2_000_000)
        );
    }
}
