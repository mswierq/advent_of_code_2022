use std::io::{BufRead, BufReader};
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

#[derive(Clone)]
struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn horizontal(start_y: i32) -> Self {
        Self {
            points: vec![
                Point::new(2, start_y),
                Point::new(3, start_y),
                Point::new(4, start_y),
                Point::new(5, start_y),
            ],
        }
    }

    fn plus(start_y: i32) -> Self {
        Self {
            points: vec![
                Point::new(2, start_y + 1),
                Point::new(3, start_y),
                Point::new(3, start_y + 1),
                Point::new(3, start_y + 2),
                Point::new(4, start_y + 1),
            ],
        }
    }

    fn inverted_l(start_y: i32) -> Self {
        Self {
            points: vec![
                Point::new(2, start_y),
                Point::new(3, start_y),
                Point::new(4, start_y),
                Point::new(4, start_y + 1),
                Point::new(4, start_y + 2),
            ],
        }
    }

    fn vertical(start_y: i32) -> Self {
        Self {
            points: vec![
                Point::new(2, start_y),
                Point::new(2, start_y + 1),
                Point::new(2, start_y + 2),
                Point::new(2, start_y + 3),
            ],
        }
    }

    fn square(start_y: i32) -> Self {
        Self {
            points: vec![
                Point::new(2, start_y),
                Point::new(2, start_y + 1),
                Point::new(3, start_y),
                Point::new(3, start_y + 1),
            ],
        }
    }

    fn collides_with(&self, rhs: &Self) -> bool {
        self.points
            .iter()
            .any(|self_p| rhs.points.iter().any(|rhs_p| self_p == rhs_p))
    }

    fn move_right(&self) -> Self {
        // the last point is always the far-right
        if self.points.last().unwrap().x < 6 {
            return Self {
                points: self
                    .points
                    .iter()
                    .map(|p| p.clone() + Point::new(1, 0))
                    .collect(),
            };
        }
        self.clone()
    }

    fn move_left(&self) -> Self {
        // the first point is always the far-left
        if self.points.first().unwrap().x > 0 {
            return Self {
                points: self
                    .points
                    .iter()
                    .map(|p| p.clone() - Point::new(1, 0))
                    .collect(),
            };
        }
        self.clone()
    }

    fn move_down(&self) -> Self {
        Self {
            points: self
                .points
                .iter()
                .map(|p| p.clone() - Point::new(0, 1))
                .collect(),
        }
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let rocks = [
        |y| Rock::horizontal(y),
        |y| Rock::plus(y),
        |y| Rock::inverted_l(y),
        |y| Rock::vertical(y),
        |y| Rock::square(y),
    ];
    let gusts = reader.lines().next().unwrap().unwrap();
    let mut new_rock_builder = rocks.iter().cycle();
    let mut next_gust = gusts.chars().cycle();

    let mut tower: Vec<Rock> = vec![];
    tower.reserve(2023);
    // a special rock that is the floor
    tower.push(Rock {
        points: (0..7).map(|x| Point::new(x, 0)).collect(),
    });
    let mut tower_height = 0;

    while tower.len() < 2023 {
        let mut current_rock = new_rock_builder.next().unwrap()(tower_height + 5);
        let mut keep_falling = true;
        while keep_falling {
            let mut moved_rock = current_rock.move_down();
            keep_falling = !tower
                .iter()
                .rev()
                .any(|rock| moved_rock.collides_with(rock));
            if keep_falling {
                current_rock = moved_rock;
            } else {
                continue;
            }
            let gust = next_gust.next().unwrap();
            match gust {
                '>' => moved_rock = current_rock.move_right(),
                '<' => moved_rock = current_rock.move_left(),
                _ => panic!("Unknown gust! {}", gust),
            }
            if tower
                .iter()
                .rev()
                .all(|rock| !moved_rock.collides_with(rock))
            {
                current_rock = moved_rock;
            }
        }
        tower_height = tower_height.max(current_rock.points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y);
        tower.push(current_rock);
    }

    println!("Tower height: {}", tower_height);

    Ok(())
}
