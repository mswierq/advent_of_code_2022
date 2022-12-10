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

    let sum_signal_strength: Register = register_trace
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(idx, register)| {
            let cycle = idx + 1;
            if cycle < 221 {
                *register * cycle as Register
            } else {
                0
            }
        })
        .sum();

    println!("Sum signal strength: {}", sum_signal_strength);

    Ok(())
}
