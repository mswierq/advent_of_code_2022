use crate::Matter::Sand;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::{fs::File, io};

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

struct Path {
    points: Vec<Point>,
}

impl Path {
    fn get_all_rock_points(&self) -> Vec<Point> {
        let mut rock_points = vec![];
        for i in 0..self.points.len() - 1 {
            let diff_point = self.points[i + 1].clone() - self.points[i].clone();
            rock_points.push(self.points[i].clone());
            if diff_point.x < -1 {
                // go left
                for j in 1..diff_point.x.abs() {
                    rock_points.push(self.points[i].clone() - Point::new(j, 0));
                }
            }
            if diff_point.x > 1 {
                // go right
                for j in 1..diff_point.x.abs() {
                    rock_points.push(self.points[i].clone() + Point::new(j, 0));
                }
            }
            if diff_point.y > 1 {
                // go down
                for j in 1..diff_point.y.abs() {
                    rock_points.push(self.points[i].clone() + Point::new(0, j));
                }
            }
            if diff_point.y < -1 {
                // go up
                for j in 1..diff_point.y.abs() {
                    rock_points.push(self.points[i].clone() - Point::new(0, j));
                }
            }
        }
        rock_points.push(self.points.last().unwrap().clone());
        rock_points
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" -> ");
        Ok(Self {
            points: split
                .map(|point| point.parse::<Point>().expect("Couldn't parse a Point!"))
                .collect(),
        })
    }
}

#[derive(PartialEq, Clone)]
enum Matter {
    Rock,
    Air,
    Sand,
}

struct CaveScan {
    map: Vec<Vec<Matter>>,
    offset: usize,
}

impl CaveScan {
    fn new(paths: Vec<Path>) -> Self {
        let rock_points = paths.iter().map(|path| path.get_all_rock_points()).fold(
            Vec::<Point>::new(),
            |mut acc, mut x| {
                acc.append(&mut x);
                acc
            },
        );
        let right_edge = rock_points
            .iter()
            .max_by(|&x, &y| x.x.cmp(&y.x))
            .expect("Expected to get a right-most point")
            .x;
        let bottom_edge = rock_points
            .iter()
            .max_by(|&x, &y| x.y.cmp(&y.y))
            .expect("Expected to get a lowest point")
            .y;
        let n_rows = bottom_edge as usize + 3;
        let n_cols = right_edge as usize + 1;

        let offset = n_cols / 3;
        let mut map = vec![vec![Matter::Air; n_cols]; n_rows];

        for rock in rock_points {
            map[rock.y as usize][rock.x as usize - offset] = Matter::Rock
        }

        for bottom in map[n_rows - 1].iter_mut() {
            *bottom = Matter::Rock;
        }

        Self { map, offset }
    }

    fn produce_sand(&mut self) -> usize {
        while self.map[0][500 - self.offset] == Matter::Air {
            let mut sand = Point::new(500 - self.offset as i32, 0);
            while (sand.y as usize) < self.map.len() - 1 {
                // until the void hasn't been reach
                let x = sand.x as usize;
                let y = sand.y as usize;
                if self.map[y + 1][x] == Matter::Air {
                    sand = sand + Point::new(0, 1);
                } else if self.map[y + 1][x - 1] == Matter::Air {
                    sand = sand + Point::new(-1, 1);
                } else if self.map[y + 1][x + 1] == Matter::Air {
                    sand = sand + Point::new(1, 1);
                } else {
                    self.map[y][x] = Sand;
                    break;
                }
            }
        }
        self.map.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .map(|col| if *col == Sand { 1 } else { 0 })
                .sum::<usize>()
        })
    }

    fn to_string(&self) -> String {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x {
                        Matter::Rock => '#',
                        Matter::Air => '.',
                        Matter::Sand => 'o',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .fold(String::new(), |acc, x| acc + &x)
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut cave = CaveScan::new(
        reader
            .lines()
            .map(|line| Path::from_str(&line.unwrap()).unwrap())
            .collect(),
    );

    println!("N sand units: {}", cave.produce_sand());

    //println!("{}", cave.to_string());

    Ok(())
}
