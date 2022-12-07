use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut stacks = HashMap::<usize, Vec<char>>::new();
    let mut ids = Vec::new();
    let stack_re = Regex::new(r"(\[[A-Z]\]|[ ]{3})(?:[ ]?|$)").unwrap();
    let id_re = Regex::new(r"\s*(\d)\s*").unwrap();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        for (i, cap) in stack_re.captures_iter(&line).enumerate() {
            let elem = &cap[1];
            let mut stack = stacks.entry(i).or_default();
            if elem.starts_with("[") {
                stack.push(elem.chars().nth(1).unwrap());
            }
        }

        for id in id_re.captures_iter(&line) {
            ids.push(id[1].parse::<usize>().unwrap());
        }
    }
    let mut ordered_stacks: BTreeMap<_, _> = stacks
        .into_iter()
        .map(|(key, stack)| {
            let new_id = *ids.get(key).unwrap();
            (new_id, stack)
        })
        .collect();

    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    while let Some(Ok(line)) = lines.next() {
        let captures = re.captures(&line).unwrap();
        let n = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let mut moved_stack = ordered_stacks.remove(&from).unwrap();
        ordered_stacks.insert(from, moved_stack.split_off(n));
        let mut to_stack = ordered_stacks.remove(&to).unwrap();
        moved_stack.append(&mut to_stack);
        ordered_stacks.insert(to, moved_stack);
    }

    let result: String = ordered_stacks
        .into_iter()
        .map(|(_, stack)| *stack.get(0).unwrap())
        .collect();

    println!("Result: {}", result);

    Ok(())
}
