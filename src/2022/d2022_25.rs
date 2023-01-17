use crate::tools;
static FNAME: &str = "./input/2022/2022-25-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
}

fn part1(lines: &Vec<String>) {
  let sum = lines.iter().map(|x| snafu_to_dec(x)).sum();
  let ans = dec_to_snafu(sum);
  println!("Bob's SNAFU: {ans}");
}

fn snafu_to_dec(snafu: &str) -> i64 {
  let mut sum = 0;
  for c in snafu.chars() {
    sum *= 5;
    sum += match c {
      '=' => -2,
      '-' => -1,
      '0' => 0,
      '1' => 1,
      '2' => 2,
      _ => panic!("Unexpected snafu character: {c}"),
    };
  }
  return sum;
}

fn dec_to_snafu(mut dec: i64) -> String {
  let mut s = String::new();

  while dec != 0 {
    let rem = (dec + 2) % 5 - 2;
    dec = (dec - rem) / 5;
    s.insert(0, match rem {
      -2 => '=',
      -1 => '-',
      0 => '0',
      1 => '1',
      2 => '2',
      _ => panic!("Unexpected decimal remainder: {rem}"),
    });
  }
  return s;
}
