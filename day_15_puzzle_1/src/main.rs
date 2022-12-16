use regex::Regex;
use std::collections::HashSet;
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

fn calculate_n_covered_spots(devices: Vec<DevicePair>, y_pos: i32) -> usize {
    let mut visited_points = HashSet::<Point>::new();
    for sensor_beacon in &devices {
        let sensor = sensor_beacon.sensor.clone();
        let beacon = sensor_beacon.beacon.clone();
        let distance = sensor.distance(&beacon);
        let dist_from_y_pos = (y_pos - sensor.y).abs();
        if dist_from_y_pos <= distance {
            for x in 0..(distance - dist_from_y_pos + 1) {
                let new_point = Point::new(sensor.x + x, y_pos);
                if devices
                    .iter()
                    .all(|pair| pair.sensor != new_point && pair.beacon != new_point)
                {
                    visited_points.insert(new_point);
                }
            }
            for x in 1..(distance - dist_from_y_pos + 1) {
                let new_point = Point::new(sensor.x - x, y_pos);
                if devices
                    .iter()
                    .all(|pair| pair.sensor != new_point && pair.beacon != new_point)
                {
                    visited_points.insert(new_point);
                }
            }
        }
    }
    visited_points.len()
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let device_pairs = reader
        .lines()
        .map(|s| DevicePair::from_str(&s.unwrap()).expect("Couldn't parse sensor and beacon pair!"))
        .collect::<Vec<_>>();

    println!(
        "Beacons cannot be present at these many places: {}",
        calculate_n_covered_spots(device_pairs, 2000000)
    );

    Ok(())
}
