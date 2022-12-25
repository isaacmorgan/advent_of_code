use crate::aoc::OP::{ADDX, NOOP};
use crate::tools;
static FNAME: &str = "./input/2022/2022-10-01.txt";

#[derive(Debug)]
enum OP {
  NOOP,
  ADDX(i32),
}
pub fn main() {
  let input = tools::load(FNAME);
  let hist = part1(&input);
  part2(&input, &hist);
}

fn part1(lines: &Vec<String>) -> Vec<(i32, i32)> {
  let ops = parse(lines);

  // Save cycle and value at end of cycle
  let mut hist: Vec<(i32, i32)> = Vec::new();
  let mut x = 1;
  let mut c = -1;
  hist.push((c, x));

  for op in ops {
    match op {
      NOOP => c += 1,
      ADDX(n) => {
        c += 2;
        x += n;
        hist.push((c, x));
      },
    }
  }

  let mut sum = 0;
  for cycle in (20..240).step_by(40) {
    let X = hist.iter().filter(|(c, x)| c < &cycle).last().unwrap().1;
    let score = cycle * X;
    sum += score;
    println!("Cycle: {cycle} Score: {score}");
  }

  println!("Total: {sum}");

  return hist;
}

fn part2(lines: &Vec<String>, hist: &Vec<(i32, i32)>) {
  let mut ln = [' '; 6*40];
  for cycle in 0..6*40 {
    let col = cycle%40 + 1;
    let x = hist.iter().filter(|(c, x)| c < &cycle).last().unwrap().1;
    if x > col - 3 && x <= col {
      ln[cycle as usize] = '*';
    }
  }

  for i in 0..6 {
    println!("{}", ln[i*40..(i+1)*40].iter().collect::<String>());
  }
}

fn parse(lines: &Vec<String>) -> Vec<OP> {
  let mut ops = Vec::new();
  for ln in lines {
    match ln.split_at(4) {
      ("noop", _) => ops.push(NOOP),
      ("addx", v) => ops.push(ADDX(v.trim().parse().unwrap())),
      _ => panic!("Unrecognized input: {ln}"),
    }
  }

  return ops;
}
