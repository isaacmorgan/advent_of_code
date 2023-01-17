use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-23-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let mut map = parse(lines);
  let mut rules_offset = 0;
  print_map(&map);

  for i in 0..10 {
    let mut proposed_moves = HashMap::new();
    let mut dest_cnt = HashMap::new();
    for elf in &map {
      let p = get_proposed_move(&elf, &map, rules_offset);
      match p {
        Some(x) => {
          dest_cnt.insert(x, dest_cnt.get(&x).unwrap_or(&0) + 1);
          proposed_moves.insert(elf, x);
        },
        None => (),
      }
    }
    let mut next_map = HashSet::new();
    for elf in &map {
      if let Some(p) = proposed_moves.get(elf) {
        if dest_cnt.get(&p).unwrap() == &1 {
          next_map.insert(*p);
        } else {
          next_map.insert(*elf);
        }
      } else {
        next_map.insert(*elf);
      }
    }

    map = next_map;
    print_map(&map);
    rules_offset += 1;
  }

  let x_min = map.iter().map(|x| x.0).min().unwrap();
  let x_max = map.iter().map(|x| x.0).max().unwrap();
  let y_min = map.iter().map(|x| x.1).min().unwrap();
  let y_max = map.iter().map(|x| x.1).max().unwrap();

  let empty = (y_max - y_min + 1)*(x_max - x_min + 1) - map.len() as i32;

  println!("Part 1 Empty Tiles: {empty}");

}

fn part2(lines: &Vec<String>) {
  let mut map = parse(lines);
  let mut rules_offset = 0;

  let mut cnt = 0;
  loop {
    cnt += 1;
    let mut did_move = false;
    let mut proposed_moves = HashMap::new();
    let mut dest_cnt = HashMap::new();
    for elf in &map {
      let p = get_proposed_move(&elf, &map, rules_offset);
      match p {
        Some(x) => {
          dest_cnt.insert(x, dest_cnt.get(&x).unwrap_or(&0) + 1);
          proposed_moves.insert(elf, x);
        },
        None => (),
      }
    }
    let mut next_map = HashSet::new();
    for elf in &map {
      if let Some(p) = proposed_moves.get(elf) {
        if dest_cnt.get(&p).unwrap() == &1 {
          next_map.insert(*p);
          did_move = true;
        } else {
          next_map.insert(*elf);
        }
      } else {
        next_map.insert(*elf);
      }
    }

    map = next_map;
    rules_offset += 1;
    if !did_move {
      break;
    }
  }

  println!("Part 2 First Round No Movement: {cnt}");
}

fn print_map(map: &HashSet<(i32, i32)>) {
  let x_min = map.iter().map(|x| x.0).min().unwrap();
  let x_max = map.iter().map(|x| x.0).max().unwrap();
  let y_min = map.iter().map(|x| x.1).min().unwrap();
  let y_max = map.iter().map(|x| x.1).max().unwrap();
  println!();
  for r in x_min..=x_max {
    for c in y_min..=y_max {
      if map.contains(&(r,c)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!();
}

fn get_proposed_move(elf: &(i32, i32), map: &HashSet<(i32, i32)>, rule_offset: i32) -> Option<(i32, i32)> {
  if is_empty(&elf, &map) {
    return None;
  }
  for i in 0..4 {
    let dir = match (rule_offset + i)%4 {
      0 => 'N',
      1 => 'S',
      2 => 'W',
      3 => 'E',
      _ => panic!("Unexpected rule offset: {rule_offset} + {i}"),
    };
    if is_dir_empty(&elf, &dir, &map) {
      return match dir {
        'N' => Some((elf.0 - 1, elf.1)),
        'S' => Some((elf.0 + 1, elf.1)),
        'W' => Some((elf.0, elf.1 - 1)),
        'E' => Some((elf.0, elf.1 + 1)),
        _ => panic!("Unexpected dir: {dir}"),
      };
    }
  }

  return None;
}

fn is_empty(elf: &(i32, i32), map: &HashSet<(i32, i32)>) -> bool {
  for r in (elf.0 - 1)..=(elf.0 + 1) {
    for c in (elf.1 - 1)..=(elf.1 + 1) {
      if r == elf.0 && c == elf.1 {
        continue;
      }
      if map.contains(&(r,c)) {
        return false;
      }
    }
  }
  return true;
}

fn is_dir_empty(elf: &(i32, i32), dir: &char, map: &HashSet<(i32, i32)>) -> bool {
  let delta = match dir {
    'N' => [(-1, -1), (-1, 0), (-1, 1)],
    'S' => [(1, -1), (1, 0), (1, 1)],
    'E' => [(-1, 1), (0, 1), (1, 1)],
    'W' => [(-1, -1), (0, -1), (1, -1)],
    _ => panic!("Unexpected direction: {dir}"),
  };
  for (r,c) in delta {
    if map.contains(&(elf.0 + r, elf.1 + c)) {
      return false;
    }
  }
  return true;
}

fn parse(lines: &Vec<String>) -> HashSet<(i32, i32)> {
  let mut map = HashSet::new();
  for (row, ln) in lines.iter().enumerate() {
    for (col, c) in ln.chars().enumerate() {
      if c == '#' {
        map.insert((row as i32, col as i32));
      }
    }
  }
  return map;
}
