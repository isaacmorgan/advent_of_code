use std::cmp::{min, Ordering};
use crate::tools;
static FNAME: &str = "./input/2022/2022-13-01.txt";

enum Type {
  list,
  int,
}

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let pairs = parse(lines);
  let mut sum = 0;
  for (i, p) in pairs.iter().enumerate() {
    dbg!(&p);
    let x = compare(&p.0, &p.1);
    if x < 0 {
      sum += i + 1;
    }
  }
  dbg!(&pairs);
  println!("Part 1 sum: {sum}");
}

fn part2(lines: &Vec<String>) {
  let pairs = parse(lines);
  let mut packets: Vec<&String> = pairs.iter().map(|x| vec![&x.0, &x.1]).flatten().collect();
  let dp1 = "[[2]]".to_string();
  let dp2 = "[[6]]".to_string();
  packets.push(&dp1);
  packets.push(&dp2);
  packets.sort_by(|a, b| match compare(a.as_str(),b.as_str()) {
    -1 => Ordering::Less,
    0 => Ordering::Equal,
    1 => Ordering::Greater,
    _ => panic!("Unexpected output from compare()"),
  });
  dbg!(&packets);
  let code: Vec<i32> = packets.iter().enumerate().filter(|(i, x)| &&&dp1 == x || &&&dp2 == x).map(|(i, x)| i as i32 + 1).collect::<Vec<i32>>();
  println!("Part 2 code: {}", code.first().unwrap() * code.last().unwrap());
}

fn get_type(ln: &str) -> Type {
  match &ln[..1] {
    "[" => Type::list,
    _ => Type::int,
  }
}

fn split_list(ln: &str) -> Vec<&str> {
  let mut list_items = Vec::new();
  let mut li0 = 1;
  let mut li1 = 0;
  let mut depth = 0;
  for (i, c) in ln.chars().enumerate() {
    match c {
      '[' => depth += 1,
      ']' => {
        depth -= 1;
        if depth == 0 {
          li1 = i;
          if li1 != li0 {
            list_items.push(&ln[li0..li1]);
          }
          li0 = i + 1;
        }
      },
      ',' => {
        if depth == 1 {
          li1 = i;
          list_items.push(&ln[li0..li1]);
          li0 = i + 1;
        }
      },
      _ => {},
    }
  }
  return list_items;
}

fn compare(a: &str, b: &str) -> i32 {
  let ta = get_type(a);
  let tb = get_type(b);
  return match (ta, tb) {
    (Type::int, Type::int) => compare_ints(a, b),
    (Type::int, Type::list) => {
      let mut na = "[".to_string();
      na.push_str(a);
      na.push(']');
      compare_lists(&na, b)
    },
    (Type::list, Type::int) => {
      let mut nb = "[".to_string();
      nb.push_str(b);
      nb.push(']');
      compare_lists(a, &nb)
    },
    (Type::list, Type::list) => compare_lists(a, b),
  }
}

fn compare_lists(a: &str, b: &str) -> i32 {
  let la = split_list(a);
  let lb = split_list(b);
  let n = min(la.len(), lb.len());
  for i in 0..n {
    let cmp = compare(la.get(i).unwrap(), lb.get(i).unwrap());
    if cmp != 0 {
      return cmp;
    }
  }
  return (a.len() as i32 - b.len() as i32).signum();
}

fn compare_ints(a: &str, b: &str) -> i32 {
  println!("{} : {}", &a, &b);
  return (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).signum();
}

fn parse(lines: &Vec<String>) -> Vec<(String, String)> {
  let mut pairs = Vec::new();
  let mut ln1 = "";
  let mut ln2 = "";
  for (i, ln) in lines.iter().enumerate() {
    match i%3 {
      0 => ln1 = ln,
      1 => ln2 = ln,
      2 => pairs.push((ln1.to_string(), ln2.to_string())),
      _ => (),
    }
  }
  // Push the last set bc the last line isn't parsed.
  pairs.push((ln1.to_string(), ln2.to_string()));
  return pairs;
}
