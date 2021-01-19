use crate::tools;
use std::collections::HashSet;
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-06.txt";
pub fn main() {
    part1();
    part2();
}

fn part1() {
  let groups = load();
  let cnt: usize = groups.iter()
    .map(|g| {
      g.iter()
        .map(|p| p.chars())
        .flatten()
        .collect::<HashSet<char>>()
    })
      .map(|x| x.len())
      .sum();
  println!("Sum of counts {:?}", cnt);
}

fn part2() {
  let groups = load();
  let cnt: usize = groups.iter()
      .map(|g| {
        let mut s = HashSet::new();
        for x in g.get(0).unwrap().chars() {
          s.insert(x);
        }
        for i in 1..g.len() {
            let g2 = g.get(i).unwrap().chars().collect_vec();
            s = s.iter().filter(|x| g2.contains(*x)).map(|x| x.clone()).collect::<HashSet<char>>();
        }
        s.len()
      })
      .sum();
  println!("Sum of counts {:?}", cnt);
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