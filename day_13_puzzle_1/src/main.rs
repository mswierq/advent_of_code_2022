use crate::DataType::{Integer, List};
use crate::Order::{InOrder, Inconclusive, OutOfOrder};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fs::File, io};

enum DataType {
    Integer(i32),
    List(Vec<DataType>),
}

fn print_data(data: &DataType) {
    match data {
        Integer(v) => print!("{}", v),
        List(list) => {
            print!("[");
            for l in list {
                print_data(l);
                print!(",");
            }
            print!("]");
        }
    }
}

#[derive(PartialEq)]
enum Order {
    InOrder,
    Inconclusive,
    OutOfOrder,
}

impl DataType {
    fn check_order(&self, right: &Self) -> Order {
        match (self, right) {
            (Integer(l), Integer(r)) => {
                if l == r {
                    return Inconclusive;
                }
                if l < r {
                    return InOrder;
                }
                return OutOfOrder;
            }
            (Integer(l), List(_)) => List(vec![Integer(*l)]).check_order(right),
            (List(_), Integer(r)) => self.check_order(&List(vec![Integer(*r)])),
            (List(l), List(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    let result = l.check_order(r);
                    if result == InOrder || result == OutOfOrder {
                        return result;
                    }
                }
                if l.len() > r.len() {
                    return OutOfOrder;
                }
                if l.len() < r.len() {
                    return InOrder;
                }
                return Inconclusive;
            }
        }
    }
}

fn push_element(mut depths: VecDeque<Vec<DataType>>, element: &str) -> VecDeque<Vec<DataType>> {
    if !element.is_empty() {
        depths
            .front_mut()
            .expect("Expected at least one list!")
            .push(Integer(
                element.parse().expect("Couldn't parse an integer!"),
            ));
    }
    depths
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depths: VecDeque<Vec<DataType>> = VecDeque::new();
        let mut start = 0;
        for (idx, c) in s.chars().enumerate() {
            if c == '[' {
                depths.push_front(vec![]);
                start = idx + 1;
            } else if c == ']' {
                depths = push_element(depths, &s[start..idx]);
                let list = depths.pop_front().expect("Expected at least one list!");
                if let Some(mut upper_list) = depths.pop_front() {
                    upper_list.push(List(list));
                    depths.push_front(upper_list)
                } else {
                    return Ok(List(list));
                }
                start = idx + 1;
            } else if c == ',' {
                depths = push_element(depths, &s[start..idx]);
                start = idx + 1;
            }
        }
        Err(format!("Couldn't parse {} into DataType!", s))
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut packet_pairs = Vec::<(DataType, DataType)>::new();
    while let Some(Ok(line)) = lines.next() {
        let first = DataType::from_str(&line).expect("Couldn't parse the first packet!");
        let second = DataType::from_str(&lines.next().expect("Expected a second packet!").unwrap())
            .expect("Couldn't parse the second packet!");
        packet_pairs.push((first, second));
        lines.next();
    }

    let sum_of_ids: usize = packet_pairs
        .iter()
        .enumerate()
        .map(|(idx, (first, second))| {
            if first.check_order(second) != OutOfOrder {
                idx + 1
            } else {
                0
            }
        })
        .sum();

    println!("Sum of ids = {}", sum_of_ids);

    Ok(())
}
