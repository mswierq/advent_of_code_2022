use std::collections::{BTreeMap, HashMap, VecDeque};
use std::error;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Rem};
use std::str::FromStr;
use std::{fs::File, io};

// base value that contains all the common divisors from the input
const BASE_VALUE: u64 = 2 * 3 * 5 * 7 * 9 * 11 * 13 * 17 * 19;

#[derive(Clone)]
struct WorryLevel {
    value: u64,
}

impl From<u64> for WorryLevel {
    fn from(u: u64) -> Self {
        Self {
            value: u % BASE_VALUE,
        }
    }
}

impl Add for WorryLevel {
    type Output = WorryLevel;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value + rhs.value) % BASE_VALUE,
        }
    }
}

impl Mul for WorryLevel {
    type Output = WorryLevel;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value) % BASE_VALUE,
        }
    }
}

impl Rem<u64> for WorryLevel {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        self.value % rhs
    }
}

struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Box<dyn Fn(WorryLevel) -> i32>,
    inspected_counter: u64,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(";");

        let items: VecDeque<WorryLevel> = split.next().expect("Expected items list!")[16..]
            .split(", ")
            .map(|item| WorryLevel::from(item.parse::<u64>().expect("Couldn't parse single item")))
            .collect();

        let mut op_split = split.next().expect("Wrong operation format!")[21..].split(" ");
        let operator: fn(WorryLevel, WorryLevel) -> WorryLevel =
            match op_split.next().expect("Expected to get an operator!") {
                "+" => |x: WorryLevel, y: WorryLevel| x + y,
                "*" => |x: WorryLevel, y: WorryLevel| x * y,
                _ => panic!("Unknown operator!"),
            };
        let operation_arg = op_split
            .next()
            .expect("Expected to get an operation argument!")
            .to_owned();
        let operation = Box::new(move |x: WorryLevel| -> WorryLevel {
            if operation_arg == "old" {
                return operator(x.clone(), x);
            }
            let arg = WorryLevel::from(
                operation_arg
                    .parse::<u64>()
                    .expect("Parsing the big int has failed!"),
            );
            return operator(x, arg);
        });

        let divisor = u64::from_str(&split.next().expect("Expected to get a divisor!")[19..])
            .expect("Parsing the divisor has failed!");
        let true_id = split
            .next()
            .expect("Expected to get Monkey Id for a true case")[25..]
            .parse::<i32>()?;
        let false_id = split
            .next()
            .expect("Expected to get Monkey Id for a false case")[26..]
            .parse::<i32>()?;
        let test = Box::new(move |x: WorryLevel| {
            if x % divisor == 0 {
                return true_id;
            }
            false_id
        });

        Ok(Self {
            items,
            operation,
            test,
            inspected_counter: 0,
        })
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkeys = BTreeMap::<i32, Monkey>::new();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Monkey") {
            let end_of_id = line.len() - 1;
            let id: i32 = line[7..end_of_id]
                .parse()
                .expect("Couldn't read monkey's id!");
            let mut monkey_description = String::new();
            while let Some(Ok(line)) = lines.next() {
                if line.is_empty() {
                    break;
                }
                monkey_description += &(line.trim().to_owned() + ";");
            }
            monkeys.insert(
                id,
                Monkey::from_str(&monkey_description).expect("Couldn't parse the monkey!"),
            );
        }
    }

    let monkey_ids: Vec<i32> = monkeys.keys().map(|x| x.clone()).collect();
    for _ in 0..10000 {
        for id in &monkey_ids {
            let mut moved_items = HashMap::<i32, VecDeque<WorryLevel>>::new();
            {
                let monkey = monkeys.get_mut(id).unwrap();
                while let Some(item) = monkey.items.pop_front() {
                    let worry_level = (monkey.operation)(item);
                    let pass_to_monkey = (monkey.test)(worry_level.clone());
                    moved_items
                        .entry(pass_to_monkey)
                        .or_default()
                        .push_back(worry_level);
                    monkey.inspected_counter += 1;
                }
            }
            for (other_monkey_id, items) in moved_items.iter_mut() {
                monkeys.entry(*other_monkey_id).and_modify(|other_monkey| {
                    other_monkey.items.append(items);
                });
            }
        }
    }

    let mut inspected_items_counters: Vec<u64> = monkeys
        .iter()
        .map(|(_, monkey)| monkey.inspected_counter)
        .collect();

    inspected_items_counters.sort();

    println!(
        "Monkey business: {}",
        inspected_items_counters
            .iter()
            .rev()
            .take(2)
            .product::<u64>()
    );

    Ok(())
}
