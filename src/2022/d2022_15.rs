use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::future::IntoFuture;
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-15-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  let row = 2_000_000;
  part1(&input, row);
  part2(&input);
}

fn part1(lines: &Vec<String>, row: i32) {
  let data = parse(lines);
  let mut spans = Vec::new();

  for sensor in &data {
    match check_no_beacon(sensor, row) {
      Some(s) => spans.push(s),
      None => (),
    }
  }

  spans = consolidate_spans(&spans);

  // Count beacons within span
  let beacons = get_unique_beacons_at_row(&data, row);
  let mut beacon_cnt = 0;
  for o in beacons {
    for s in &spans {
      if (s.0..=s.1).contains(&o.1) {
        beacon_cnt += 1;
      }
    }
  }
  //dbg!(&beacon_cnt);

  // Count spans
  let mut span_cnt = 0;
  for s in &spans {
    span_cnt += s.1 - s.0 + 1;
  }

  //dbg!(&data);
  print_row(&spans);
  println!("Part 1: Number of empty spots: {}", span_cnt - beacon_cnt);
}

fn part2(lines: &Vec<String>) {
  let MIN_X = 0;
  let MAX_X = 4_000_000;
  let data = parse(lines);
  let mut spans = Vec::new();
  let (mut x, mut y) = (0, 0);
  'loop_row:
  for row in MIN_X..=MAX_X {
    spans.clear();
    for sensor in &data {
      match check_no_beacon(sensor, row) {
        Some(s) => spans.push(s),
        None => (),
      }
    }
    spans = consolidate_spans(&spans);

    for s in &spans {
      if s.0 <= MIN_X && s.1 >= MAX_X {
        continue 'loop_row;
      }
      if s.0 <= MIN_X && s.1 >= MIN_X && s.1 < MAX_X {
        x = s.1 + 1;
        y = row;
        break 'loop_row;
      }
      if s.1 >= MAX_X && s.0 <= MAX_X && s.0 > MIN_X {
        x = s.0 - 1;
        y = row;
        break 'loop_row;
      }
    }
  }
  println!("Part 2: Empty space at {}, {}\n Tuning frequency: {}", x, y, MAX_X as u64 * x as u64 + y as u64);
}

fn get_unique_beacons_at_row(sensors: &Vec<(i32, i32, i32, i32)>, row: i32) -> HashSet<(i32, i32)> {
  let mut beacons = HashSet::new();
  for s in sensors {
    if s.3 == row {
      beacons.insert((s.2, s.3));
    }
  }
  return beacons;
}

fn consolidate_spans(spans: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
  let mut mut_spans = spans.clone();
  for i in 0..mut_spans.len() {
    for j in (i+1)..mut_spans.len() {
      let a = mut_spans.get(i).unwrap();
      let b = mut_spans.get(j).unwrap();
      if a.1 >= b.0 && a.0 <= b.1 {
        let min = min(a.0, b.0);
        let max = max(a.1, b.1);
        let c = mut_spans.get_mut(j).unwrap();
        c.0 = min;
        c.1 = max;
      }
    }
  }
  /*for (i, a) in mut_spans.iter().enumerate() {
    for b in mut_spans.iter_mut().skip(i+1) {
      if a.1 >= b.0 && a.0 <= b.1 {
        b.0 = min(a.0, b.0);
        b.1 = max(a.1, b.1);
      }
    }
  }
*/
  let mut final_spans = Vec::new();
  'outer:
  for (i, a) in mut_spans.iter().enumerate() {
    for b in mut_spans.iter().skip(i+1) {
      if a.1 >= b.0 && a.0 <= b.1 {
        continue 'outer;
      }
    }
    final_spans.push(a.clone());
  }

  return final_spans;
}

fn check_no_beacon(sensor: &(i32, i32, i32, i32), row: i32) -> Option<(i32, i32)> {
  let d = (sensor.0 - sensor.2).abs() + (sensor.1 - sensor.3).abs();
  let dy = (row - sensor.1).abs();

  let min_x = sensor.0 - d + dy;
  let max_x = sensor.0 + d - dy;

  return if min_x <= max_x {
    Some((min_x, max_x))
  } else {
    None
  }
}

fn print_row(spans: &Vec<(i32, i32)>) {
  'outer:
  for x in -4..27 {
    for s in spans {
      if (s.0..=s.1).contains(&x) {
        print!("#");
        continue 'outer;
      }
    }
    print!(".");
  }
  println!();
}

fn parse(lines: &Vec<String>) -> Vec<(i32, i32, i32, i32)>{
  let mut data = Vec::new();

  for ln in lines {
    let a = ln.split_once(": closest beacon is at x=").unwrap();
    let (b, sy) = a.0.split_once(", y=").unwrap();
    let (_, sx) = b.split_once("=").unwrap();
    let (bx, by) = a.1.split_once(", y=").unwrap();
    data.push((
      sx.parse().unwrap(),
      sy.parse().unwrap(),
      bx.parse().unwrap(),
      by.parse().unwrap(),
    ));
  }

  return data;
}
