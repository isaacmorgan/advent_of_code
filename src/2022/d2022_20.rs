use std::collections::{HashMap, LinkedList};
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-20-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let values = parse(&lines);
  let index: Vec<usize> = (0..values.len()).collect();
  let n = values.len() - 1;
  let mut list = index.clone();
  //dbg!(&list);
  for i in &index {
    let (j, _) = list.iter().enumerate().find(|(x, l)| l == &i).unwrap();
    let d = ((values[*i] + j as i32).rem_euclid(n as i32)) as usize;
    //dbg!(&v, &d, &i);
    list.remove(j);
    list.insert(d, *i);
    //dbg!(&list);
  }
  //dbg!(&values);
  //dbg!(&list);

  // TOOD: Update this to find the index of 0 then use that
  // Find 0th item
  let (i0, _) = values.iter().enumerate().find(|(x, l)| l == &&0).unwrap();
  let (i, _) = list.iter().enumerate().find(|(x, l)| l == &&i0).unwrap();

  // 0th + 1000, 2000, 3000
  let d_1k = ((1000 + i as i32).rem_euclid((n+1) as i32)) as usize;
  let d_2k = ((2000 + i as i32).rem_euclid((n+1) as i32)) as usize;
  let d_3k = ((3000 + i as i32).rem_euclid((n+1) as i32)) as usize;

  let grove_sum = values[list[d_1k]] + values[list[d_2k]] + values[list[d_3k]];
  println!("Numbers at 1k: {}, 2k: {}, 3k: {}", values[list[d_1k]], values[list[d_2k]], values[list[d_3k]]);
  println!("Part 1 sum: {grove_sum}");
}

fn part2(lines: &Vec<String>) {
  let values = parse(&lines);
  let values: Vec<i64> = values.iter().map(|x| (*x as i64) * 811589153).collect();
  let index: Vec<usize> = (0..values.len()).collect();
  let n = values.len() - 1;
  let mut list = index.clone();
  //dbg!(&list);
  for i_ in 0..10 {
    for i in &index {
      let (j, _) = list.iter().enumerate().find(|(x, l)| l == &i).unwrap();
      let d = ((values[*i] + j as i64).rem_euclid(n as i64)) as usize;
      //dbg!(&v, &d, &i);
      list.remove(j);
      list.insert(d, *i);
      //dbg!(&list);
    }
    //dbg!(&values);
    //dbg!(&list);
  }
  // Find 0th item
  let (i0, _) = values.iter().enumerate().find(|(x, l)| l == &&0).unwrap();
  let (i, _) = list.iter().enumerate().find(|(x, l)| l == &&i0).unwrap();

  // 0th + 1000, 2000, 3000
  let d_1k = ((1000 + i as i32).rem_euclid((n+1) as i32)) as usize;
  let d_2k = ((2000 + i as i32).rem_euclid((n+1) as i32)) as usize;
  let d_3k = ((3000 + i as i32).rem_euclid((n+1) as i32)) as usize;

  let grove_sum = values[list[d_1k]] + values[list[d_2k]] + values[list[d_3k]];
  println!("Numbers at 1k: {}, 2k: {}, 3k: {}", values[list[d_1k]], values[list[d_2k]], values[list[d_3k]]);
  println!("Part 1 sum: {grove_sum}");
}

fn parse(lines: &Vec<String>) -> Vec<i32> {
  lines.iter().map(|x| x.parse::<i32>().unwrap()).collect()
}
