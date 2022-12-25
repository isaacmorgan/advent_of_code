use crate::aoc::Operation::{Add, Multiply, Power};
use crate::tools;
static FNAME: &str = "./input/2022/2022-11-01.txt";
static WORRY_DIV: u64 = 3;

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

#[derive(Debug, Copy, Clone)]
enum Operation {
  Add(u64),
  Multiply(u64),
  Power(u32),
}

#[derive(Debug)]
struct Monkey {
  items: Vec<u64>,
  operation: Operation,
  test: u64,
  next: (usize, usize),
  inspections: u64,
}

fn part1(lines: &Vec<String>) {
  let mut monkeys = parse(lines);
  //dbg!(&monkeys);
  let mod_val = monkeys.iter().map(|m| m.test).product();
  for _ in 0..20 {
    for s in 0..monkeys.len() {
      step(&mut monkeys, s, WORRY_DIV, mod_val);
    }
  }
  //dbg!(&monkeys);

  monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
  let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
  println!("Part 1 Monkey Business: {monkey_business}");
}

fn part2(lines: &Vec<String>) {
  let mut monkeys = parse(lines);
  //dbg!(&monkeys);
  let mod_val = monkeys.iter().map(|m| m.test).product();
  for _ in 0..10000 {
    for s in 0..monkeys.len() {
      step(&mut monkeys, s, 1, mod_val);
    }
  }
  //dbg!(&monkeys);

  monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
  let monkey_business = monkeys[0].inspections as u64 * monkeys[1].inspections as u64;
  println!("Part 2 Monkey Business: {monkey_business}");
}

fn step(monkeys: &mut Vec<Monkey>, ind: usize, worry_div: u64, mod_val: u64) {
  // Get inspection result
  let worry = &monkeys[ind].items;

  // Operate on items
  let new_worry: Vec<u64> = match monkeys[ind].operation {
    Add(x) => worry.iter().map(|w| (w + x)/worry_div).collect(),
    Multiply(x) => worry.iter().map(|w| (w * x)/worry_div).collect(),
    Power(x) => worry.iter().map(|w| w.pow(x)/worry_div).collect(),
  };

  // Check for destination monkey
  let destination: Vec<usize> = new_worry.iter()
      .map(|x| if x%monkeys[ind].test == 0 {monkeys[ind].next.0} else {monkeys[ind].next.1} ).collect();

  //dbg!(&new_worry, &destination);
  // Send to destination monkey
  for (w, d) in new_worry.iter().zip(destination) {
    monkeys[d].items.push(*w%mod_val);
  }
  monkeys[ind].inspections += monkeys[ind].items.len() as u64;
  monkeys[ind].items.clear();
}

fn parse(lines: &Vec<String>) -> Vec<Monkey> {
  let mut monkeys = Vec::new();
  let mut items: Vec<u64>;
  let mut operation;
  let mut test: u64;
  let mut next;
  for i in 0..(lines.len()+1)/7 { // +1 because there's no blank line at end of file (\n\n)
    items = lines.get(i*7 + 1).unwrap()
        .strip_prefix("  Starting items: ").unwrap()
        .split(", ").map(|x| x.parse::<u64>().unwrap()).collect();

    operation = match lines.get(i*7 + 2).unwrap()
        .strip_prefix("  Operation: new = old ").unwrap()
        .split_once(" ").unwrap() {
      ("+", x) => Operation::Add(x.parse().unwrap()),
      ("*", "old") => Operation::Power(2),
      ("*", x) => Operation::Multiply(x.parse().unwrap()),
      _ => panic!("Unrecognized Operation."),
    };

    test = lines.get(i*7 + 3).unwrap()
        .strip_prefix("  Test: divisible by ").unwrap()
        .parse().unwrap();

    let to_a = lines.get(i*7 + 4).unwrap()
        .strip_prefix("    If true: throw to monkey ").unwrap()
        .parse().unwrap();

    let to_b = lines.get(i*7 + 5).unwrap()
        .strip_prefix("    If false: throw to monkey ").unwrap()
        .parse().unwrap();

    next = (to_a, to_b);

    monkeys.push( Monkey {
      items: items.clone(),
      operation: operation.clone(),
      test: test.clone(),
      next: next.clone(),
      inspections: 0,
    })
  }
  return monkeys;
}
