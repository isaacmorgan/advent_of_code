use crate::tools;

static FNAME: &str = "./input/2020/2020-05.txt";

pub fn main() {
  part1();
  part2();
}

fn part1() {
  let pass = load();
  let m = pass.iter()
    .map(|x| pass2seat(x))
    .max()
    .unwrap();
  println!("Max value: {}", m);
}

fn part2() {
  let pass = load();
  let mut m: Vec<i32> = pass.iter()
    .map(|x| pass2seat(x))
    .collect();
  m.sort();
  for i in 1..m.len() {
    if m[i] - m[i-1] == 2 {
      println!("Seat number: {}", m[i] - 1);
      break;
    }
  }
}

fn pass2seat(p: &str) -> i32 {
  let mut row = 0;
  let mut col = 0;
  let two: i32 = 2;
  for i in (0..=6 as usize) {
    if &p[i..=i] == "B" {
      row += two.pow(6 - i as u32);
    }
  }
  for i in (7..=9 as usize) {
    if &p[i..=i] == "R" {
      col += two.pow(9 - i as u32);
    }
  }
  return row * 8 + col;
}

fn load() -> Vec<String> {
  tools::load(FNAME)
}