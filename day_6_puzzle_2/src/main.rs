use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut message = String::new();
    reader
        .read_line(&mut message)
        .expect("Reading a message has failed!");

    let mut start_of_packet_idx = 0;
    if message.len() >= 14 {
        for idx in 0..message.len() - 14 {
            let set: HashSet<_> = message[idx..idx + 14].chars().collect();
            if set.len() == 14 {
                start_of_packet_idx = idx + 14;
                break;
            }
        }
    }

    println!("Marker can after {}th char!", start_of_packet_idx);

    Ok(())
}
