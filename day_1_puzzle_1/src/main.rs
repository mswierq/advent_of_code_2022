use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut current_elf_calories = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            max_calories = std::cmp::max(max_calories, current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    println!("Max calories: {}", max_calories);

    Ok(())
}
