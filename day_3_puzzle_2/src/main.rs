use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::ops::BitAnd;
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut score: i32 = 0;

    let mut lines_iter = reader.lines();
    while let (Some(Ok(line0)), Some(Ok(line1)), Some(Ok(line2))) =
        (lines_iter.next(), lines_iter.next(), lines_iter.next())
    {
        let rucksack0 = line0.chars().collect::<HashSet<char>>();
        let rucksack1 = line1.chars().collect::<HashSet<char>>();
        let rucksack2 = line2.chars().collect::<HashSet<char>>();
        let common = rucksack0.bitand(&rucksack1).bitand(&rucksack2);
        score += common
            .iter()
            .map(|x| {
                if *x >= 'A' && *x <= 'Z' {
                    *x as i32 - 'A' as i32 + 27
                } else {
                    *x as i32 - 'a' as i32 + 1
                }
            })
            .sum::<i32>();
    }

    println!("SCORE: {}", score);

    Ok(())
}
