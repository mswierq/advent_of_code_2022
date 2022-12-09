use std::collections::HashSet;
use std::default::Default;
use std::io::{BufRead, BufReader};
use std::{fs::File, io};

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Point {
    // Moves to new position are stays at the same one to
    // to maintain the point hovered or next to the other Point
    fn move_next_to(self, other: &Self) -> Self {
        let diff = other.clone() - self.clone();
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            if diff.x == 2 && diff.y == 0 {
                // right
                return Self {
                    x: self.x + 1,
                    y: self.y,
                };
            } else if diff.x == -2 && diff.y == 0 {
                // left
                return Self {
                    x: self.x - 1,
                    y: self.y,
                };
            } else if diff.x == 0 && diff.y == 2 {
                // up
                return Self {
                    x: self.x,
                    y: self.y + 1,
                };
            } else if diff.x == 0 && diff.y == -2 {
                // down
                return Self {
                    x: self.x,
                    y: self.y - 1,
                };
            } else if (diff.x > 1 && diff.y == 1) || (diff.x == 1 && diff.y > 1) {
                // diag up-right
                return Self {
                    x: self.x + 1,
                    y: self.y + 1,
                };
            } else if (diff.x < -1 && diff.y == 1) || (diff.x == -1 && diff.y > 1) {
                // diag up-left
                return Self {
                    x: self.x - 1,
                    y: self.y + 1,
                };
            } else if (diff.x > 1 && diff.y == -1) || (diff.x == 1 && diff.y < -1) {
                // diag down-right
                return Self {
                    x: self.x + 1,
                    y: self.y - 1,
                };
            } else if (diff.x < -1 && diff.y == -1) || (diff.x == -1 && diff.y < -1) {
                // diag down-left
                return Self {
                    x: self.x - 1,
                    y: self.y - 1,
                };
            }
        }
        self
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut visited_points = HashSet::<Point>::new();
    let mut tail = Point::default();
    let mut head = Point::default();
    for result_line in reader.lines() {
        let line = result_line.expect("Couldn't read a line!");
        let mut split = line.split(" ");
        let direction = split.next().expect("Didn't get a direction!");
        let n_steps = split
            .next()
            .expect("Didn't get number of steps!")
            .parse::<i32>()
            .expect("Couldn't parse number of steps!");
        for _ in 0..n_steps {
            match direction {
                "R" => {
                    head.x += 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "U" => {
                    head.y += 1;
                }
                "D" => {
                    head.y -= 1;
                }
                _ => panic!("Unknown direction: {}!", direction),
            }
            tail = tail.move_next_to(&head);
            visited_points.insert(tail.clone());
        }
    }

    println!("N visited points: {}", visited_points.len());

    Ok(())
}
