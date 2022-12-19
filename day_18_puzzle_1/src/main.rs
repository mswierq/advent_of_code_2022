use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::{fs::File, io};

#[derive(Clone, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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
            z: split
                .next()
                .expect("Missing Z!")
                .parse::<i32>()
                .expect("Couldn't parse Z"),
        })
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut cubes: HashMap<_, _> = reader
        .lines()
        .map(|s| (Point::from_str(&s.unwrap()).unwrap(), 6))
        .collect();

    let positions: Vec<Point> = cubes.keys().map(|k| k.clone()).collect();

    for position in positions {
        let neighboring = [
            position.clone() + Point::new(1, 0, 0),
            position.clone() + Point::new(-1, 0, 0),
            position.clone() + Point::new(0, 1, 0),
            position.clone() + Point::new(0, -1, 0),
            position.clone() + Point::new(0, 0, 1),
            position.clone() + Point::new(0, 0, -1),
        ];
        let n_adjacent: usize = neighboring
            .iter()
            .map(|p| {
                if cubes.contains_key(&p) {
                    return 1;
                }
                0
            })
            .sum();
        *cubes.get_mut(&position).unwrap() -= n_adjacent;
    }

    println!(
        "The surface area is {}",
        cubes.iter().map(|(_, n_sides)| n_sides).sum::<usize>()
    );

    Ok(())
}
