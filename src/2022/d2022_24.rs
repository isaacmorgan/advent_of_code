use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::tools;
static FNAME: &str = "./input/2022/2022-24-01.txt";

#[derive(Debug)]
struct Level {
  start: (usize, usize),
  finish: (usize, usize),
  x_min: usize,
  x_max: usize,
  y_min: usize,
  y_max: usize,
}

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let (mut map, level) = parse(lines);
  let res = count_steps(&map, &level, 0);
  println!("Part 1: Finished in {res} steps.");
}

fn part2(lines: &Vec<String>) {
  let (mut map, mut level) = parse(lines);
  let res_a = count_steps(&map, &level, 0);
  let new_finish = level.start.clone();
  level.start = level.finish;
  level.finish = new_finish;
  let res_b = count_steps(&map, &level, res_a as usize);
  let new_finish = level.start.clone();
  level.start = level.finish;
  level.finish = new_finish;
  let res_c = count_steps(&map, &level, res_b as usize);
  println!("Part 2: Finished in {res_c} steps.");
}

fn part3(lines: &Vec<String>) {
  let (mut map, level) = parse(lines);
  dbg!(&level, &map);
  let mut player = level.start;
  let mut states = BinaryHeap::new();
  let mut visited = HashSet::new();
  states.push(Reverse((0usize, player.clone())));
  let mut last_time: i32 = -1;
  'outer:
  loop {
    let mut s = states.pop().unwrap().0;
    dbg!(&s.0, &s.1);
    if s.0 as i32 > last_time {
      step_map(&mut map, &level);
      last_time = s.0 as i32;
    }
    for dp in  [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
      let mut p = (s.1.0 as i32 + dp.0, s.1.1 as i32 + dp.1);
      //dbg!(&p);
      if p.0 as usize == level.finish.0 && p.1 as usize == level.finish.1 {
        println!("Part 1: Win in {}", s.0 + 1);
        break 'outer;
      }
      if (p.0 != level.start.0 as i32 || p.1 != level.start.1 as i32)
          && (p.0 < level.y_min as i32
          || p.1 < level.x_min as i32
          || p.0 > level.y_max as i32
          || p.1 > level.x_max as i32) {
        //println!("Bounds");
        continue;
      }
      if visited.contains(&(s.0 + 1, p.0 as usize, p.1 as usize)) {
        //println!("Visited");
        continue;
      }
      if map.contains(&(p.0 as usize, p.1 as usize, '^'))
          || map.contains(&(p.0 as usize, p.1 as usize, 'v'))
          || map.contains(&(p.0 as usize, p.1 as usize, '<'))
          || map.contains(&(p.0 as usize, p.1 as usize, '>'))
      {
        //println!("Blocked");
        continue;
      }
      //dbg!(&p);
      states.push(Reverse((s.0 + 1, (p.0 as usize, p.1 as usize))));
      visited.insert((s.0+1, p.0 as usize, p.1 as usize));
    }
  }

}

fn part4(lines: &Vec<String>) {
  let (mut map, level) = parse(lines);
  let map_up: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '^').map(|x| (x.0, x.1)).collect();
  let map_down: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == 'v').map(|x| (x.0, x.1)).collect();
  let map_left: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '<').map(|x| (x.0, x.1)).collect();
  let map_right: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '>').map(|x| (x.0, x.1)).collect();

  let mut player = level.start;
  let mut states = BinaryHeap::new();
  let mut visited = HashSet::new();
  states.push(Reverse((0usize, player.clone())));
  'outer:
  loop {
    let mut s = states.pop().unwrap().0;
    dbg!(&s.0, &s.1);

    for dp in  [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
      let mut p = (s.1.0 as i32 + dp.0, s.1.1 as i32 + dp.1);
//dbg!(&p);
      if p.0 as usize == level.finish.0 && p.1 as usize == level.finish.1 {
        println!("Part 1: Win in {}", s.0 + 1);
        break 'outer;
      }
      if (p.0 != level.start.0 as i32 || p.1 != level.start.1 as i32)
          && (p.0 < level.y_min as i32
          || p.1 < level.x_min as i32
          || p.0 > level.y_max as i32
          || p.1 > level.x_max as i32) {
//println!("Bounds");
        continue;
      }
      if visited.contains(&(s.0 + 1, p.0 as usize, p.1 as usize)) {
//println!("Visited");
        continue;
      }
      if !is_spot_open(&map_up, p.clone(), &level, s.0 as i32 + 1, '^')
          | !is_spot_open(&map_down, p.clone(), &level, s.0 as i32 + 1, 'v')
          | !is_spot_open(&map_left, p.clone(), &level, s.0 as i32 + 1, '<')
          | !is_spot_open(&map_right, p.clone(), &level, s.0 as i32 + 1, '>')
      {
//println!("Blocked");
        continue;
      }
//dbg!(&p);
      states.push(Reverse((s.0 + 1, (p.0 as usize, p.1 as usize))));
      visited.insert((s.0+1, p.0 as usize, p.1 as usize));
    }
  }
}

fn count_steps(map: &Vec<(usize, usize, char)>, level: &Level, offset: usize) -> i32 {
  let map_up: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '^').map(|x| (x.0, x.1)).collect();
  let map_down: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == 'v').map(|x| (x.0, x.1)).collect();
  let map_left: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '<').map(|x| (x.0, x.1)).collect();
  let map_right: Vec<(usize, usize)> = map.iter().filter(|x| x.2 == '>').map(|x| (x.0, x.1)).collect();

  let mut player = level.start;
  let mut states = BinaryHeap::new();
  let mut visited = HashSet::new();
  states.push(Reverse((offset, player.clone())));
  'outer:
  loop {
    let mut s = states.pop().unwrap().0;
    //dbg!(&s.0, &s.1);

    for dp in  [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
      let mut p = (s.1.0 as i32 + dp.0, s.1.1 as i32 + dp.1);
//dbg!(&p);
      if p.0 as usize == level.finish.0 && p.1 as usize == level.finish.1 {
        //println!("Part 1: Win in {}", s.0 + 1);
        //break 'outer;
        return s.0 as i32 + 1;
      }
      if (p.0 != level.start.0 as i32 || p.1 != level.start.1 as i32)
          && (p.0 < level.y_min as i32
          || p.1 < level.x_min as i32
          || p.0 > level.y_max as i32
          || p.1 > level.x_max as i32) {
//println!("Bounds");
        continue;
      }
      if visited.contains(&(s.0 + 1, p.0 as usize, p.1 as usize)) {
//println!("Visited");
        continue;
      }
      if !is_spot_open(&map_up, p.clone(), &level, s.0 as i32 + 1, '^')
          | !is_spot_open(&map_down, p.clone(), &level, s.0 as i32 + 1, 'v')
          | !is_spot_open(&map_left, p.clone(), &level, s.0 as i32 + 1, '<')
          | !is_spot_open(&map_right, p.clone(), &level, s.0 as i32 + 1, '>')
      {
//println!("Blocked");
        continue;
      }
//dbg!(&p);
      states.push(Reverse((s.0 + 1, (p.0 as usize, p.1 as usize))));
      visited.insert((s.0+1, p.0 as usize, p.1 as usize));
    }
  }
}

fn is_spot_open(map: &Vec<(usize, usize)>, mut pos: (i32, i32), level: &Level, steps: i32, dir: char) -> bool {
  //dbg!(&pos, &steps, &dir);
  if pos.0 == level.start.0 as i32 && pos.1 == level.start.1 as i32 {
    return true;
  }
  match dir {
    '^' => pos.0 += steps,
    'v' => pos.0 -= steps,
    '<' => pos.1 += steps,
    '>' => pos.1 -= steps,
    _ => (),
  };
  //dbg!(&pos);
  let pos_u = ((pos.0 - level.y_min as i32).rem_euclid((level.y_max - level.y_min + 1) as i32) as usize + level.y_min,
               (pos.1 - level.x_min as i32).rem_euclid((level.x_max - level.x_min + 1) as i32) as usize + level.x_min);
  //dbg!(&pos_u);

  let is_open = !map.contains(&pos_u);
  //dbg!(&is_open);
  is_open
}

fn step_map(map: &mut Vec<(usize, usize, char)>, level: &Level) {
  for (r, c, dir) in map {
    match dir {
      '^' => if *r > level.y_min {*r -= 1;} else {*r = level.y_max;},
      'v' => if *r < level.y_max {*r += 1;} else {*r = level.y_min;},
      '<' => if *c > level.x_min {*c -= 1;} else {*c = level.x_max;},
      '>' => if *c < level.x_max {*c += 1;} else {*c = level.x_min;},
      _ => panic!("Unrecognized blizzard direction: {dir}"),
    }
  }
}

fn parse(lines: &Vec<String>) -> (Vec<(usize, usize, char)>, Level) {
  let mut map = Vec::new();
  let (mut start, mut finish) = ((0, 0), (0, 0));
  for (row, ln) in lines.iter().enumerate() {
    for (col, c) in ln.chars().enumerate() {
      match c {
        'v' | '^' | '>' | '<' => {map.push((row, col, c));},
        '.' => {
          if row == 0 {
            start = (row, col);
          } else {
            finish = (row, col);
          }
        },
        _ => (),
      };
    }
  }
  let level = Level {
    start,
    finish,
    x_min: 1,
    x_max: lines.first().unwrap().len() - 2,
    y_min: 1,
    y_max: lines.len() - 2,
  };

  return (map, level);
}
