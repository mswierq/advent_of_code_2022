use rug::Integer;
use std::collections::{BTreeMap, HashMap};
use std::error;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fs::File, io};

struct Monkey {
    items: Vec<Integer>,
    operation: Box<dyn Fn(&Integer) -> Integer>,
    test: Box<dyn Fn(&Integer) -> i32>,
    inspected_counter: u64,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(";");

        let items: Vec<Integer> = split.next().expect("Expected items list!")[16..]
            .split(", ")
            .map(|item| Integer::from_str(item).expect("Couldn't parse single item"))
            .collect();

        let mut op_split = split.next().expect("Wrong operation format!")[21..].split(" ");
        let operator: fn(&Integer, &Integer) -> Integer =
            match op_split.next().expect("Expected to get an operator!") {
                "+" => |x: &Integer, y: &Integer| Integer::from(x + y),
                "*" => |x: &Integer, y: &Integer| Integer::from(x * y),
                _ => panic!("Unknown operator!"),
            };
        let operation_arg = op_split
            .next()
            .expect("Expected to get an operation argument!")
            .to_owned();
        let operation = Box::new(move |x: &Integer| -> Integer {
            if operation_arg == "old" {
                return operator(x, x);
            }
            let arg = Integer::from_str(&operation_arg).expect("Parsing the big int has failed!");
            return operator(&x, &arg);
        });

        let divisor = u32::from_str(&split.next().expect("Expected to get a divisor!")[19..])
            .expect("Parsing the divisor has failed!");
        let true_id = split
            .next()
            .expect("Expected to get Monkey Id for a true case")[25..]
            .parse::<i32>()?;
        let false_id = split
            .next()
            .expect("Expected to get Monkey Id for a false case")[26..]
            .parse::<i32>()?;
        let test = Box::new(move |x: &Integer| {
            if x.is_divisible_u(divisor) {
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
    for _ in 0..20 {
        for id in &monkey_ids {
            let mut moved_items = HashMap::<i32, Vec<Integer>>::new();
            {
                let monkey = monkeys.get_mut(id).unwrap();
                for item in &monkey.items {
                    let worry_level: Integer = (monkey.operation)(item) / 3;
                    let pass_to_monkey = (monkey.test)(&worry_level);
                    moved_items
                        .entry(pass_to_monkey)
                        .or_default()
                        .push(worry_level);
                    monkey.inspected_counter += 1;
                }
                monkey.items.clear();
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
