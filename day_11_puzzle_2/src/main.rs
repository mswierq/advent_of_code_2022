use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::error;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::{fs::File, io};

#[derive(Clone)]
struct Composite {
    primes: Vec<u128>,
}

impl From<u128> for Composite {
    fn from(v: u128) -> Self {
        let mut divisor = 2;
        let mut dividend = v;
        let mut primes = vec![];
        while dividend > 1 {
            if dividend % divisor == 0 {
                dividend /= divisor;
                primes.push(divisor);
            } else {
                if divisor >= 3 {
                    divisor += 2;
                } else {
                    divisor += 1;
                }
            }
        }
        Self { primes }
    }
}

impl Composite {
    // assumes that a divisor is a prime number
    fn is_divisible(&self, prime: u128) -> bool {
        self.primes.contains(&prime)
    }
}

impl Add for Composite {
    type Output = Composite;

    fn add(self, rhs: Self) -> Self::Output {
        self.primes rhs.primes
    }
}

impl Mul for Composite {
    type Output = Composite;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            primes: &self.primes | &rhs.primes,
        }
    }
}

struct Monkey {
    items: VecDeque<Composite>,
    operation: Box<dyn Fn(Composite) -> Composite>,
    test: Box<dyn Fn(&Composite) -> i32>,
    inspected_counter: u64,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(";");

        let items: VecDeque<Composite> = split.next().expect("Expected items list!")[16..]
            .split(", ")
            .map(|item| Composite::from(item.parse::<u128>().expect("Couldn't parse single item")))
            .collect();

        let mut op_split = split.next().expect("Wrong operation format!")[21..].split(" ");
        let operator: fn(Composite, Composite) -> Composite =
            match op_split.next().expect("Expected to get an operator!") {
                "+" => |x: Composite, y: Composite| x + y,
                "*" => |x: Composite, y: Composite| x * y,
                _ => panic!("Unknown operator!"),
            };
        let operation_arg = op_split
            .next()
            .expect("Expected to get an operation argument!")
            .to_owned();
        let operation = Box::new(move |x: Composite| -> Composite {
            if operation_arg == "old" {
                return operator(x.clone(), x);
            }
            let arg = Composite::from(
                operation_arg
                    .parse::<u128>()
                    .expect("Parsing the big int has failed!"),
            );
            return operator(x, arg);
        });

        let divisor = u128::from_str(&split.next().expect("Expected to get a divisor!")[19..])
            .expect("Parsing the divisor has failed!");
        let true_id = split
            .next()
            .expect("Expected to get Monkey Id for a true case")[25..]
            .parse::<i32>()?;
        let false_id = split
            .next()
            .expect("Expected to get Monkey Id for a false case")[26..]
            .parse::<i32>()?;
        let test = Box::new(move |x: &Composite| {
            if x.is_divisible(divisor) {
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
            let mut moved_items = HashMap::<i32, VecDeque<Composite>>::new();
            {
                let monkey = monkeys.get_mut(id).unwrap();
                while let Some(item) = monkey.items.pop_front() {
                    let worry_level = (monkey.operation)(item);
                    let pass_to_monkey = (monkey.test)(&worry_level);
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
