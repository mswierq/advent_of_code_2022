use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::ops::BitAnd;
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut score: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let len = line.len();
        let fst_compartment = &line[..len / 2].chars().collect::<HashSet<char>>();
        let snd_compartment = &line[len / 2..len].chars().collect::<HashSet<char>>();
        let common = fst_compartment.bitand(snd_compartment);
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
