use crate::tools;
use std::collections::HashSet;
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-06.txt";
pub fn main() {
  part1();
}

fn part1() {
  let groups = load();
  let cnt = groups.iter()
    .map(|g| {
      g.iter()
        .map(|p| p.chars())
        .flatten()
        .collect::HashSet<char>()
    })
    .collect_vec();
  println!("{:?}", cnt);
}

fn load() -> Vec<Vec<String>> {
  let input = tools::load(FNAME);
  let mut ans = Vec::new();
  let mut group = Vec::new();
  for line in input {
    if line.is_empty() {
      ans.push(group);
      group = Vec::new();
      continue;
    }
    group.push(line);
  }
  ans.push(group);
  return ans;
}