use std::cmp::{max, min};
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-14-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let mut map = parse(lines);
  let start = [500, 0];
  let mut pos = (start[0], start[1]);
  //print_map(&map);
  loop {
    match step(&mut map, &pos) {
      (true, None) => pos = (start[0], start[1]), // Start over with new sand drop
      (true, Some((x, y))) => pos = (x, y), // Continue with current sand drop
      (false, None) => break, // Done with program
      _ => panic!("Unexpected result from Step"),
    }
  }
  let sand_count = map.iter().flatten().filter(|x| x == &&'o').count();
  println!("Sand count: {sand_count}");
}

fn part2(lines: &Vec<String>) {
  let mut map = parse(lines);
  map.push(vec!['.'; map.first().unwrap().len()]);
  map.push(vec!['#'; map.first().unwrap().len()]);
  let start = [500, 0];
  let mut pos = (start[0], start[1]);
  //print_map(&map);
  loop {
    match step(&mut map, &pos) {
      (true, None) => pos = (start[0], start[1]), // Start over with new sand drop
      (true, Some((x, y))) => pos = (x, y), // Continue with current sand drop
      (false, None) => break, // Done with program
      _ => panic!("Unexpected result from Step"),
    }
  }
  let sand_count = map.iter().flatten().filter(|x| x == &&'o').count();
  println!("Sand count: {sand_count}");
}

fn print_map(map: &Vec<Vec<char>>) {
  for r in map {
    for c in r {
      print!("{c}");
    }
    println!();
  }
}

fn step(map: &mut Vec<Vec<char>>, pos: &(usize, usize)) -> (bool, Option<(usize, usize)>) {
  let max_y = map.len();
  let max_x = map.first().unwrap().len();
  for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
    let x: i32 = pos.0 as i32 + dx;
    let y: i32 = pos.1 as i32 + dy;
    if map[pos.1][pos.0] == 'o' {
      return (false, None);
    }
    if y >= max_y as i32 {
      return (false, None);
    }
    if x < 0 || x >= max_x as i32 {
      return (false, None);
    }
    if map[y as usize][x as usize] == '.' {
      return (true, Some((x as usize, y as usize)));
    }
  }
  map[pos.1][pos.0] = 'o';
  return (true, None);
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
  let walls = lines.iter().map(|ln|
      ln.split(" -> ")
          .map(|x|
              x.split(',').map(|y| y.parse::<usize>().unwrap()).collect_vec())
          .collect_vec()
  ).collect_vec();

  let max_x = walls.iter().map(|x| x.iter().map(|y| y[0]).max()).max().unwrap().unwrap();
  let max_y = walls.iter().map(|x| x.iter().map(|y| y[1]).max()).max().unwrap().unwrap();

  let mut map = vec![vec!['.'; 2*max_x + 1]; max_y + 1];

  println!("{} {}", map.len(), map.first().unwrap().len());
  for w in walls {
    let mut x = 0;
    let mut y = 0;
    for (i, p) in w.iter().enumerate() {
      if i == 0 {
        x = p[0];
        y = p[1];
      }
      for ix in min(x, p[0])..=max(x, p[0]) {
        for iy in min(y, p[1])..=max(y, p[1]) {
          map[iy][ix] = '#';
        }
      }
      x = p[0];
      y = p[1];
    }
  }

  return map;
}
