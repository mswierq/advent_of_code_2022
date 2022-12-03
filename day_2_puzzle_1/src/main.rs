use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

#[derive(PartialEq, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for HandShape {
    fn from(symbol: char) -> Self {
        match symbol {
            'A' | 'X' => HandShape::Rock,
            'B' | 'Y' => HandShape::Paper,
            'C' | 'Z' => HandShape::Scissors,
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

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut score = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let line_chars = line.chars();
        let mut symbols = line_chars.filter(|c| *c != ' ');
        let elf_hand_shape: HandShape = symbols.next().unwrap().into();
        let my_hand_shape: HandShape = symbols.next().unwrap().into();
        score += {
            let shape_score: i32 = my_hand_shape.clone().into();
            if elf_hand_shape > my_hand_shape {
                shape_score
            } else if elf_hand_shape < my_hand_shape {
                shape_score + 6
            } else {
                shape_score + 3
            }
        };
    }

    println!("SCORE: {}", score);

    Ok(())
}
