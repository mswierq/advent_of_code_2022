use std::collections::BTreeSet;
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut calories_set = BTreeSet::new();
    let mut current_elf_calories = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            calories_set.insert(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    let mut iter = calories_set.iter().rev();
    let mut max_calories = 0;
    for _ in 0..3 {
        if let Some(calories) = iter.next() {
            max_calories += calories;
        }
    }

    println!("Sum of the first three with max calories: {}", max_calories);

    Ok(())
}
