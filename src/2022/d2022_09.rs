use std::collections::HashSet;
use crate::tools;
static FNAME: &str = "./input/2022/2022-09-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let commands = parse(lines);
  let mut hist: HashSet<(i32, i32)> = HashSet::new();
  let mut knots = vec![(0, 0); 2];
  record_knot(&knots.last().unwrap(), &mut hist);
  for cmd in commands {
    println!("cmd: {} {}", cmd.0, cmd.1);
    for _ in 0..cmd.1 {
      let (head, tail) = knots.split_at_mut(1);
      lead(cmd.0, &mut head.first_mut().unwrap());
      follow(head.first().unwrap(), &mut tail.first_mut().unwrap());
      record_knot(&tail.last().unwrap(), &mut hist);
    }
    print_map(&knots);
  }
  println!("Number of T locations: {}", hist.iter().count());
}

fn part2(lines: &Vec<String>) {
  let commands = parse(lines);
  let mut hist: HashSet<(i32, i32)> = HashSet::new();
  let mut knots = vec![(0, 0); 10];
  record_knot(&knots.last().unwrap(), &mut hist);
  for cmd in commands {
    println!("cmd: {} {}", cmd.0, cmd.1);
    for _ in 0..cmd.1 {
      lead(cmd.0, &mut knots.first_mut().unwrap());
      for i in 1..knots.len() {
        let (head, tail) = knots.split_at_mut(i);
        follow(head.last().unwrap(), &mut tail.first_mut().unwrap());
      }
      record_knot(&knots.last().unwrap(), &mut hist);
      print_map(&knots);
    }
  }
  println!("Number of T locations: {}", hist.iter().count());
}

fn record_knot(tail: &(i32, i32), hist: &mut HashSet<(i32, i32)>) {
  hist.insert(tail.clone());
}

fn print_map(knots: &Vec<(i32, i32)>) {
  let x0 = knots.iter().map(|x| x.0).min().unwrap();
  let x1 = knots.iter().map(|x| x.0).max().unwrap();
  let y0 = knots.iter().map(|x| x.1).min().unwrap();
  let y1 = knots.iter().map(|x| x.1).max().unwrap();

  let mut map = vec![vec!['.'; (x1 - x0 + 1) as usize]; (y1 - y0 + 1) as usize];

  for (i, k) in knots.iter().enumerate() {
    let c = match i {
      0 => 'H',
      _ => char::from_digit(i as u32, 10).unwrap(),
    };
    map[(k.1 - y0) as usize][(k.0 - x0) as usize] = c;
  }

  println!("");
  for r in map.iter().rev() {
    println!("{}", r.iter().cloned().collect::<String>());
  }
  println!("");
}

fn lead(dir: char, knot: &mut (i32, i32)) {
  match dir {
    'R' => knot.0 += 1,
    'L' => knot.0 -= 1,
    'U' => knot.1 += 1,
    'D' => knot.1 -= 1,
    _ => panic!("Unexpected command direction: {dir}"),
  }
}

fn follow(head: &(i32, i32), tail: &mut (i32, i32)) {
  let dx = head.0 - tail.0;
  let dy = head.1 - tail.1;
  if dx.abs() > 1 || dy.abs() > 1 {
    tail.0 += if dx > 0 { 1 } else if dx < 0 { -1 } else { 0 };
    tail.1 += if dy > 0 { 1 } else if dy < 0 { -1 } else { 0 };
  }
}

fn parse(lines: &Vec<String>) -> Vec<(char, i32)> {
  let mut cmd = Vec::new();
  for ln in lines {
    let x = match ln.split_once(" ").unwrap() {
      (c, n) => (c.chars().next().unwrap(), n.parse::<i32>().unwrap()),
      _ => panic!("Unrecognized input: {ln}"),
    };
    cmd.push(x);
  }
  return cmd;
}
