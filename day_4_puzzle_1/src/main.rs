use std::io::{prelude::*, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fs::File, io};

struct Space {
    start: i32,
    end: i32,
}

impl Space {
    fn are_containing_each_other(&self, other: &Space) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }
}

impl FromStr for Space {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = s.split("-");
        let start = edges.next().unwrap().parse::<i32>()?;
        let end = edges.next().unwrap().parse::<i32>()?;
        Ok(Space { start, end })
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut n_fully_contained: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let ranges = line.split(",");
        let mut spaces = ranges.map(|r| Space::from_str(r).unwrap());
        let space0 = spaces.next().unwrap();
        let space1 = spaces.next().unwrap();
        n_fully_contained += if space0.are_containing_each_other(&space1) {
            1
        } else {
            0
        };
    }

    println!("N fully contained spaces: {}", n_fully_contained);

    Ok(())
}
