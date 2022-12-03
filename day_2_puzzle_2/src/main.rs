use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

#[derive(PartialEq, Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for HandShape {
    fn from(symbol: char) -> Self {
        match symbol {
            'A' => HandShape::Rock,
            'B' => HandShape::Paper,
            'C' => HandShape::Scissors,
            _ => panic!("Unknown hand shape {}!", symbol),
        }
    }
}

impl Into<i32> for HandShape {
    fn into(self) -> i32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else if (*self == HandShape::Rock && *other == HandShape::Scissors)
            || (*self == HandShape::Scissors && *other == HandShape::Paper)
            || (*self == HandShape::Paper && *other == HandShape::Rock)
        {
            Some(Greater)
        } else {
            Some(Less)
        }
    }
}

#[derive(Clone)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl From<char> for RoundResult {
    fn from(symbol: char) -> Self {
        match symbol {
            'X' => RoundResult::Lose,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            _ => panic!("Unknown result symbol {}!", symbol),
        }
    }
}

impl Into<i32> for RoundResult {
    fn into(self) -> i32 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

fn find_matching_hand_shape(shape: HandShape, result: RoundResult) -> HandShape {
    let hand_shapes = [HandShape::Rock, HandShape::Paper, HandShape::Scissors];
    match result {
        RoundResult::Lose => *hand_shapes.iter().find(|x| **x < shape).unwrap(),
        RoundResult::Draw => *hand_shapes.iter().find(|x| **x == shape).unwrap(),
        RoundResult::Win => *hand_shapes.iter().find(|x| **x > shape).unwrap(),
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut score: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let line_chars = line.chars();
        let mut symbols = line_chars.filter(|c| *c != ' ');
        let hand_shape: HandShape = symbols.next().unwrap().into();
        let round_result: RoundResult = symbols.next().unwrap().into();
        let shape_score: i32 = find_matching_hand_shape(hand_shape, round_result.clone()).into();
        let result_score: i32 = round_result.into();
        score += shape_score + result_score;
    }

    println!("SCORE: {}", score);

    Ok(())
}
