use regex::Regex;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::{fs::File, io};

#[derive(Clone, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, rhs: &Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        Ok(Self {
            x: split
                .next()
                .expect("Missing X!")
                .parse::<i32>()
                .expect("Couldn't parse X"),
            y: split
                .next()
                .expect("Missing Y!")
                .parse::<i32>()
                .expect("Couldn't parse Y"),
        })
    }
}

#[derive(PartialEq)]
struct DevicePair {
    sensor: Point,
    beacon: Point,
}

impl FromStr for DevicePair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)",
        )
        .unwrap();
        let cap = re.captures(s).unwrap();
        Ok(Self {
            sensor: Point::new(cap[1].parse::<i32>()?, cap[2].parse::<i32>()?),
            beacon: Point::new(cap[3].parse::<i32>()?, cap[4].parse::<i32>()?),
        })
    }
}

fn find_distress_beacon(devices: Vec<DevicePair>, search_limit: i32) -> Point {
    for pair in &devices {
        let sensor = &pair.sensor;
        let beacon = &pair.beacon;
        let distance = sensor.distance(beacon) + 1;

        let is_in_range = |pair: &DevicePair, distress: &Point| {
            pair.sensor.distance(&distress) <= pair.sensor.distance(&pair.beacon)
        };

        // lower left
        let x_range = sensor.x - distance..sensor.x + 1;
        let y_range = sensor.y..sensor.y + distance + 1;
        for (x, y) in x_range.zip(y_range) {
            if x < 0 || x > search_limit || y < 0 || y > search_limit {
                continue;
            }
            let distress_beacon = Point::new(x, y);
            if !devices
                .iter()
                .any(|pair| is_in_range(pair, &distress_beacon))
            {
                return distress_beacon;
            }
        }

        // lower right
        let x_range = sensor.x..sensor.x + distance + 1;
        let y_range = sensor.y..sensor.y + distance + 1;
        for (x, y) in x_range.rev().zip(y_range) {
            if x < 0 || x > search_limit || y < 0 || y > search_limit {
                continue;
            }
            let distress_beacon = Point::new(x, y);
            if !devices
                .iter()
                .any(|pair| is_in_range(pair, &distress_beacon))
            {
                return distress_beacon;
            }
        }

        // upper left
        let x_range = sensor.x - distance + 1..sensor.x + 1;
        let y_range = sensor.y - distance..sensor.y + 1;
        for (x, y) in x_range.zip(y_range.rev()) {
            if x < 0 || x > search_limit || y < 0 || y > search_limit {
                continue;
            }
            let distress_beacon = Point::new(x, y);
            if !devices
                .iter()
                .any(|pair| is_in_range(pair, &distress_beacon))
            {
                return distress_beacon;
            }
        }

        // upper right
        let x_range = sensor.x..sensor.x + distance + 1;
        let y_range = sensor.y - distance..sensor.y + 1;
        for (x, y) in x_range.rev().zip(y_range.rev()) {
            if x < 0 || x > search_limit || y < 0 || y > search_limit {
                continue;
            }
            let distress_beacon = Point::new(x, y);
            if !devices
                .iter()
                .any(|pair| is_in_range(pair, &distress_beacon))
            {
                return distress_beacon;
            }
        }
    }
    Point::new(-1, -1)
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let device_pairs = reader
        .lines()
        .map(|s| DevicePair::from_str(&s.unwrap()).expect("Couldn't parse sensor and beacon pair!"))
        .collect::<Vec<_>>();

    let distress_beacon = find_distress_beacon(device_pairs, 4000000);

    println!("Found beacon: {}.{}", distress_beacon.x, distress_beacon.y);
    println!("Distress freq: {}", distress_beacon.x as u128 * 4000000 + distress_beacon.y as u128);

    Ok(())
}
