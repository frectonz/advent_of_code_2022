use std::{collections::HashMap, str::FromStr};

fn main() {
    let content = std::fs::read_to_string("../test").unwrap();
    let monkeys = content
        .split("\n\n")
        .map(|x| x.parse::<Monkey>().unwrap())
        .map(|m| (m.num, m))
        .collect::<HashMap<_, _>>();

    // part1(monkeys.clone());
    part2(monkeys);
}

/*
fn part1(mut monkeys: Vec<Money>) {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut to_be_removed = Vec::new(); // watch out
            for item in monkeys[i].items.clone() {
                monkeys[i].inspections += 1;

                let mut worry_level = match monkeys[i].operation {
                    Operation::Add(num) => item + num,
                    Operation::Multiply(num) => item * num,
                    Operation::Square => item * item,
                };

                worry_level /= 3;

                if worry_level % monkeys[i].divisor == 0 {
                    let idx = monkeys[i].true_monkey;
                    let monkey = monkeys.iter_mut().find(|m| m.num == idx).unwrap();

                    monkey.items.push(worry_level);
                    to_be_removed.push(item);
                } else {
                    let idx = monkeys[i].false_monkey;
                    let monkey = monkeys.iter_mut().find(|m| m.num == idx).unwrap();

                    monkey.items.push(worry_level);
                    to_be_removed.push(item);
                }
            }

            monkeys[i].items = monkeys[i]
                .items
                .iter()
                .skip_while(|x| to_be_removed.contains(x))
                .cloned()
                .collect();
        }
    }
    let mut sorted = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();

    sorted.sort();
    sorted.reverse();

    let monkey_business = sorted[0] * sorted[1];
    println!("Part 1: {}", monkey_business);
}
*/

fn part2(mut monkeys: HashMap<usize, Monkey>) {
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            for (item, ()) in monkeys.get(&i).unwrap().items.clone() {
                monkeys.get_mut(&i).unwrap().inspections += 1;

                let monkey = monkeys.get(&i).unwrap();

                let item = item.simplify();
                let worry_level = monkey.operation.to_item(&item).simplify();

                if monkey.operation.is_divisible(&worry_level, monkey.divisor) {
                    {
                        let idx = monkey.true_monkey;
                        let true_monkey = monkeys.get_mut(&idx).unwrap();
                        true_monkey.items.insert(worry_level, ());
                    }

                    monkeys.get_mut(&i).unwrap().items.remove(&item);
                } else {
                    {
                        let idx = monkey.false_monkey;
                        let false_monkey = monkeys.get_mut(&idx).unwrap();
                        false_monkey.items.insert(worry_level, ());
                    }

                    monkeys.get_mut(&i).unwrap().items.remove(&item);
                }
            }
        }
    }

    let mut sorted = monkeys
        .iter()
        .map(|(_, m)| m.inspections)
        .collect::<Vec<_>>();

    sorted.sort();
    sorted.reverse();

    dbg!(sorted);
    // let monkey_business = sorted[0] * sorted[1];
    // println!("Part 2: {}", monkey_business);
}

#[derive(Debug, Clone)]
struct Monkey {
    num: usize,
    items: HashMap<Item, ()>,
    operation: Operation,
    divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Item {
    Num(usize),
    Multiply(Box<Item>, Box<Item>),
    Add(Box<Item>, Box<Item>),
}

impl Item {
    fn is_divisible(&self, divisor: usize) -> bool {
        match self {
            Item::Num(num) => num % divisor == 0,
            Item::Multiply(num1, num2) => num1.is_divisible(divisor) && num2.is_divisible(divisor),
            Item::Add(num1, num2) => num1.is_divisible(divisor) && num2.is_divisible(divisor),
        }
    }

    fn simplify(&self) -> Item {
        match self {
            Item::Num(num) => Item::Num(*num),
            Item::Multiply(num1, num2) => {
                let num1 = num1.simplify();
                let num2 = num2.simplify();

                match (num1, num2) {
                    (Item::Num(1), num) => num,
                    (num, Item::Num(1)) => num,
                    (Item::Num(0), _) => Item::Num(0),
                    (_, Item::Num(0)) => Item::Num(0),
                    (Item::Num(num1), Item::Num(num2)) => match num1.checked_mul(num2) {
                        Some(num) => Item::Num(num),
                        None => {
                            Item::Multiply(Box::new(Item::Num(num1)), Box::new(Item::Num(num2)))
                        }
                    },
                    (num1, num2) => Item::Multiply(Box::new(num1), Box::new(num2)),
                }
            }
            Item::Add(num1, num2) => {
                let num1 = num1.simplify();
                let num2 = num2.simplify();

                match (num1, num2) {
                    (Item::Num(0), num) => num,
                    (num, Item::Num(0)) => num,
                    (Item::Num(num1), Item::Num(num2)) => match num1.checked_add(num2) {
                        Some(num) => Item::Num(num),
                        None => Item::Add(Box::new(Item::Num(num1)), Box::new(Item::Num(num2))),
                    },
                    (num1, num2) => Item::Add(Box::new(num1), Box::new(num2)),
                }
            }
        }
    }
}

impl Operation {
    fn is_divisible(&self, item: &Item, divisor: usize) -> bool {
        let item_divisible = match item {
            Item::Num(num) => num % divisor == 0,
            Item::Multiply(num1, num2) => {
                let num1_divisible = num1.is_divisible(divisor);
                if !num1_divisible {
                    false
                } else {
                    num2.is_divisible(divisor) && true
                }
            }
            Item::Add(num1, num2) => {
                let num1_divisible = num1.is_divisible(divisor);
                if !num1_divisible {
                    false
                } else {
                    num2.is_divisible(divisor) && true
                }
            }
        };

        if !item_divisible {
            return false;
        }

        let operation_divisible = match self {
            Operation::Add(num) => num % divisor == 0,
            Operation::Multiply(num) => num % divisor == 0,
            Operation::Square => true,
        };

        item_divisible && operation_divisible
    }

    fn to_item(&self, item: &Item) -> Item {
        use Item::*;
        match (self, item.clone()) {
            (Operation::Add(num), Item::Num(num2)) => Add(Box::new(Num(num2)), Box::new(Num(*num))),
            (Operation::Multiply(num), Item::Num(num2)) => {
                Multiply(Box::new(Num(*num)), Box::new(Num(num2)))
            }
            (Operation::Square, Item::Num(num)) => Multiply(Box::new(Num(num)), Box::new(Num(num))),
            (Operation::Add(num), Item::Multiply(num1, num2)) => {
                Add(Multiply(num1, num2).into(), Num(*num).into())
            }
            (Operation::Multiply(num), Item::Multiply(num1, num2)) => {
                Multiply(Multiply(num1, num2).into(), Num(*num).into())
            }
            (Operation::Square, Item::Multiply(num1, num2)) => Multiply(
                Multiply(num1.clone(), num2.clone()).into(),
                Multiply(num1, num2).into(),
            ),
            (Operation::Add(num), Item::Add(num1, num2)) => {
                Add(Add(num1, num2).into(), Num(*num).into())
            }
            (Operation::Multiply(num), Item::Add(num1, num2)) => {
                Multiply(Add(num1, num2).into(), Num(*num).into())
            }
            (Operation::Square, Item::Add(num1, num2)) => Multiply(
                Add(num1.clone(), num2.clone()).into(),
                Add(num1, num2).into(),
            ),
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut num = lines[0].split("Monkey ");
        num.next();
        let num = num
            .next()
            .unwrap()
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();

        let mut items = lines[1].split("Starting items: ");
        items.next();
        let items = items.next().unwrap().trim();
        let items = items
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .map(|x| (Item::Num(x), ()))
            .collect::<HashMap<_, _>>();

        let mut operation = lines[2].split("Operation: new = ");
        operation.next();
        let operation = operation.next().unwrap().split(' ').collect::<Vec<_>>();

        use Operation::*;
        let operation = match operation[..] {
            ["old", "*", "old"] => Square,
            ["old", "*", num] => Multiply(num.parse::<usize>().unwrap()),
            ["old", "+", num] => Add(num.parse::<usize>().unwrap()),
            _ => panic!("unknown operation"),
        };

        let mut divisor = lines[3].split("Test: divisible by ");
        divisor.next();
        let divisor = divisor.next().unwrap().parse::<usize>().unwrap();

        let mut true_monkey = lines[4].split("If true: throw to monkey ");
        true_monkey.next();
        let true_monkey = true_monkey.next().unwrap().parse::<usize>().unwrap();

        let mut false_monkey = lines[5].split("If false: throw to monkey ");
        false_monkey.next();
        let false_monkey = false_monkey.next().unwrap().parse::<usize>().unwrap();

        Ok(Self {
            num,
            items,
            operation,
            divisor,
            true_monkey,
            false_monkey,
            inspections: 0,
        })
    }
}
