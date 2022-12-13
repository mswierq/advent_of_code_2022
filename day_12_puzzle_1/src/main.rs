use std::default::Default;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Div};
use std::{fs::File, io};

#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
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

impl Div for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct HeightMap {
    map: Vec<Vec<char>>,
    start: Point,
    destination: Point,
    n_cols: usize,
    n_rows: usize,
}

impl HeightMap {
    fn new(map: Vec<Vec<char>>) -> Self {
        let mut start = Point::default();
        let mut destination = Point::default();
        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, square) in row.iter().enumerate() {
                if *square == 'S' {
                    start = Point::new(col_idx, row_idx);
                }
                if *square == 'E' {
                    destination = Point::new(col_idx, row_idx);
                }
            }
        }
        let n_cols = map[0].len();
        let n_rows = map.len();
        Self {
            map,
            start,
            destination,
            n_cols,
            n_rows,
        }
    }

    fn get_height(&self, point: &Point) -> i16 {
        let symbol = self.map[point.y][point.x];
        if symbol == 'S' {
            return 'a' as i16;
        }
        if symbol == 'E' {
            return 'z' as i16;
        }
        return symbol as i16;
    }
}

fn climb(height_map: &HeightMap, current_point: Point, previous_point: Point) -> usize {
    if current_point == height_map.destination {
        return 0;
    }
    let current_height = height_map.get_height(&current_point);
    let go_right_steps = {
        if current_point.x < height_map.n_cols {
            let next_point = Point::new(current_point.x + 1, current_point.y);
            if height_map.get_height(&next_point) - current_height <= 1
                && next_point != previous_point
            {
                return climb(height_map, next_point, current_point);
            }
        }
        usize::MAX
    };

    let go_left_steps = {
        if current_point.x > 0 {
            let next_point = Point::new(current_point.x - 1, current_point.y);
            if height_map.get_height(&next_point) - current_height <= 1
                && next_point != previous_point
            {
                return climb(height_map, next_point, current_point);
            }
        }
        usize::MAX
    };

    let go_up_steps = {
        if current_point.y > 0 {
            let next_point = Point::new(current_point.x, current_point.y - 1);
            if height_map.get_height(&next_point) - current_height <= 1
                && next_point != previous_point
            {
                return climb(height_map, next_point, current_point);
            }
        }
        usize::MAX
    };

    let go_down_steps = {
        if current_point.y < height_map.n_rows {
            let next_point = Point::new(current_point.x, current_point.y + 1);
            if height_map.get_height(&next_point) - current_height <= 1
                && next_point != previous_point
            {
                return climb(height_map, next_point, current_point);
            }
        }
        usize::MAX
    };

    go_right_steps
        .min(go_left_steps)
        .min(go_up_steps)
        .min(go_down_steps)
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let map: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let height_map = HeightMap::new(map);

    let n_steps = climb(&height_map, height_map.start, height_map.start);

    println!("N steps are needed: {}", n_steps);

    Ok(())
}
