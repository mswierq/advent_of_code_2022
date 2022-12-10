use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fs::File, io};

type Register = i64;

enum Instruction {
    Noop,
    Addx { value: Register },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let instruction = split.next().expect("Empty instruction!");
        if instruction == "noop" {
            return Ok(Instruction::Noop);
        } else if instruction == "addx" {
            return Ok(Instruction::Addx {
                value: split
                    .next()
                    .expect("Expected an argument!")
                    .parse::<Register>()
                    .map_err(|err| err.to_string())?,
            });
        }
        Err(format!("Unknown instruction {}!", instruction))
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut register_trace = vec![1 as Register];
    for result_line in reader.lines() {
        let line = result_line.expect("Couldn't read a line!");
        let instruction = Instruction::from_str(&line)
            .expect(&format!("Couldn't read the instruction: {}", line));

        let last_register_value = *register_trace.last().unwrap();
        match instruction {
            Instruction::Noop => register_trace.push(last_register_value),
            Instruction::Addx { value } => {
                register_trace.push(last_register_value);
                register_trace.push(last_register_value + value);
            }
        }
    }

    let mut register = register_trace.iter();
    for _ in 0..6 {
        let mut crt_line = String::new();
        for pixel_idx in 0..40 {
            let sprite_pos = *register.next().unwrap_or(&0);
            if pixel_idx >= sprite_pos - 1 && pixel_idx <= sprite_pos + 1 {
                crt_line.push('#');
            } else {
                crt_line.push('.');
            }
        }
        println!("{}", crt_line);
    }

    Ok(())
}
