use std::collections::HashMap;
use crate::tools;
static FNAME: &str = "./input/2022/2022-21-01.txt";

#[derive(Debug)]
enum Operation {
  Constant(i64),
  Add(String, String),
  Subtract(String, String),
  Multiply(String, String),
  Divide(String, String),
  Monkey_Constant(String),
  Equality(String, String),
}

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let monkeys = parse(lines);
  let result = solve_monkey(&monkeys, "root");
  println!("Part 1: {result}");
}

fn part2(lines: &Vec<String>) {
  let mut monkeys = parse(lines);
  let root = monkeys.get("root").unwrap();
  let op = match root {
    Operation::Add(a, b)
    | Operation::Subtract(a, b)
    | Operation::Multiply(a, b)
    | Operation::Divide(a, b) => Operation::Equality(a.to_string(), b.to_string()),
    _ => panic!("Unrecognized root operation"),
  };
  monkeys.insert("root", op);
  monkeys.insert("humn", Operation::Monkey_Constant("x".to_string()));

  let result = monkey_math(&monkeys, "root");
  println!("Part 2: {:?}", result);
}

fn monkey_math(monkeys: &HashMap<&str, Operation>, monkey: &str) -> Operation {
  let op = monkeys.get(monkey).unwrap();
  return match op {
    Operation::Constant(x) => Operation::Constant(*x),
    Operation::Add(a, b) => monkey_add(&monkey_math(&monkeys, a), &monkey_math(&monkeys, b)),
    Operation::Subtract(a, b) => monkey_subtract(&monkey_math(&monkeys, a), &monkey_math(&monkeys, b)),
    Operation::Multiply(a, b) => monkey_multiply(&monkey_math(&monkeys, a), &monkey_math(&monkeys, b)),
    Operation::Divide(a, b) => monkey_divide(&monkey_math(&monkeys, a), &monkey_math(&monkeys, b)),
    Operation::Monkey_Constant(x) => Operation::Monkey_Constant(x.to_string()),
    Operation::Equality(a, b) => monkey_equality(&monkey_math(&monkeys, a), &monkey_math(&monkeys, b)),
  }
}

fn monkey_add(a: &Operation, b: &Operation) -> Operation {
  match (a, b) {
    (Operation::Constant(A), Operation::Constant(B)) => Operation::Constant(A + B),
    (Operation::Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} + {B})")),
    (Operation::Monkey_Constant(A), Operation::Constant(B)) => Operation::Monkey_Constant(format!("({A} + {B})")),
    (Operation::Monkey_Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} + {B})")),
    _ => panic!("Attempt to monkey_add: {:?} and {:?}", a, b),
  }
}

fn monkey_subtract(a: &Operation, b: &Operation) -> Operation {
  match (a, b) {
    (Operation::Constant(A), Operation::Constant(B)) => Operation::Constant(A - B),
    (Operation::Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} - {B})")),
    (Operation::Monkey_Constant(A), Operation::Constant(B)) => Operation::Monkey_Constant(format!("({A} - {B})")),
    (Operation::Monkey_Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} - {B})")),
    _ => panic!("Attempt to monkey_subtract: {:?} and {:?}", a, b),
  }
}

fn monkey_multiply(a: &Operation, b: &Operation) -> Operation {
  match (a, b) {
    (Operation::Constant(A), Operation::Constant(B)) => Operation::Constant(A * B),
    (Operation::Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} * {B})")),
    (Operation::Monkey_Constant(A), Operation::Constant(B)) => Operation::Monkey_Constant(format!("({A} * {B})")),
    (Operation::Monkey_Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} * {B})")),
    _ => panic!("Attempt to monkey_multiply: {:?} and {:?}", a, b),
  }
}

fn monkey_divide(a: &Operation, b: &Operation) -> Operation {
  match (a, b) {
    (Operation::Constant(A), Operation::Constant(B)) => Operation::Constant(A / B),
    (Operation::Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} / {B})")),
    (Operation::Monkey_Constant(A), Operation::Constant(B)) => Operation::Monkey_Constant(format!("({A} / {B})")),
    (Operation::Monkey_Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} / {B})")),
    _ => panic!("Attempt to monkey_divide: {:?} and {:?}", a, b),
  }
}

fn monkey_equality(a: &Operation, b: &Operation) -> Operation {
  match (a, b) {
    (Operation::Constant(A), Operation::Constant(B)) => Operation::Constant(if A == B {1} else {0}),
    (Operation::Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} == {B})")),
    (Operation::Monkey_Constant(A), Operation::Constant(B)) => Operation::Monkey_Constant(format!("({A} == {B})")),
    (Operation::Monkey_Constant(A), Operation::Monkey_Constant(B)) => Operation::Monkey_Constant(format!("({A} == {B})")),
    _ => panic!("Attempt to monkey_equality: {:?} and {:?}", a, b),
  }
}

fn solve_monkey(monkeys: &HashMap<&str, Operation>, monkey: &str) -> i64 {
  let op = monkeys.get(monkey).unwrap();
  return match op {
    Operation::Constant(x) => *x,
    Operation::Add(a, b) => solve_monkey(&monkeys, a) + solve_monkey(&monkeys, b),
    Operation::Subtract(a, b) => solve_monkey(&monkeys, a) - solve_monkey(&monkeys, b),
    Operation::Multiply(a, b) => solve_monkey(&monkeys, a) * solve_monkey(&monkeys, b),
    Operation::Divide(a, b) => solve_monkey(&monkeys, a) / solve_monkey(&monkeys, b),
    Operation::Monkey_Constant(x) => panic!("Unexpected Monkey_Constant: {x}"),
    Operation::Equality(a, b) => if solve_monkey(&monkeys, a) == solve_monkey(&monkeys, b) {1} else {0},
  }
}

fn parse(lines: &Vec<String>) -> HashMap<&str, Operation> {
  let mut monkeys = HashMap::new();

  for ln in lines {
    let (name, rem) = ln.split_once(": ").unwrap();
    if !rem.contains(" ") {
      monkeys.insert(name, Operation::Constant(rem.parse().unwrap()));
    } else {
      let (a, rem) = rem.split_once(" ").unwrap();
      let (op, b) = rem.split_once(" ").unwrap();
      let operation = match op {
        "+" => Operation::Add(a.to_string(), b.to_string()),
        "-" => Operation::Subtract(a.to_string(), b.to_string()),
        "*" => Operation::Multiply(a.to_string(), b.to_string()),
        "/" => Operation::Divide(a.to_string(), b.to_string()),
        _ => panic!("Unrecognized operation: {op}"),
      };
      monkeys.insert(name, operation);
    }
  }

  return monkeys;
}
