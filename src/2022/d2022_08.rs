use std::cmp;
use itertools::enumerate;
use crate::tools;
static FNAME: &str = "./input/2022/2022-08-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let height = parse(lines);
  let visible = check_visible(height);
  let visible_count: i32 = visible.iter().map(|r| r.iter().map(|c| if *c { 1 } else { 0 }).sum::<i32>()).sum();
  println!("Number visible: {visible_count}");
}

fn part2(lines: &Vec<String>) {
  let height = parse(lines);
  let score = viewing_distance(&height);
  let ss = score.iter().map(|r| r.iter().max().unwrap()).max().unwrap();
  println!("Max score: {ss}");
  // dbg!(&height);
  // dbg!(&score);
}

fn viewing_distance(height: &Vec<Vec<i8>>) -> Vec<Vec<i32>> {
  let nr = height.len();
  let nc = height.first().unwrap().len();
  let mut dist = vec![vec![0; nc]; nr];
  for i in 0..nr {
    for j in 0..nc {
      let h = height.get(i).unwrap().get(j).unwrap();
      let mut sl = 0;
      let mut sr = 0;
      let mut su = 0;
      let mut sd = 0;
      // left
      for x in (0..j).rev() {
        sl += 1;
        if height.get(i).unwrap().get(x).unwrap() >= h {
          break;
        }
      }
      // right
      for x in j+1..nc {
        sr += 1;
        if height.get(i).unwrap().get(x).unwrap() >= h {
          break;
        }
      }
      // up
      for y in (0..i).rev() {
        su += 1;
        if height.get(y).unwrap().get(j).unwrap() >= h {
          break;
        }
      }
      // down
      for y in i+1..nr {
        sd += 1;
        if height.get(y).unwrap().get(j).unwrap() >= h {
          break;
        }
      }
      dist[i][j] = sl*sr*su*sd;
    }
  }
  return dist;
}

fn check_visible(height: Vec<Vec<i8>>) -> Vec<Vec<bool>> {
  let min_height_left: Vec<Vec<i8>> = height.iter().map(|r| {
    let mut max: i8 = -1;
    r.iter().map(|c| {
      let prev = max;
      max = cmp::max(max, *c);
      prev
    }).collect()
  }).collect();

  let min_height_right: Vec<Vec<i8>> = height.iter().map(|r| {
    let mut max: i8 = -1;
    let mut r_ = r.clone();
    r_.reverse();
    let mut r_: Vec<i8> = r_.iter().map(|c| {
      let prev = max;
      max = cmp::max(max, *c);
      prev
    }).collect();
    r_.reverse();
    r_
  }).collect();

  let mut max: Vec<i8> = vec![-1; height.first().unwrap().len()];
  let min_height_up: Vec<Vec<i8>> = height.iter().map(|r| {
    let prev = max.clone();
    r.iter().zip(prev).enumerate().map(|(i, (c, p))| {
      max[i] = cmp::max(max[i], *c);
      p
    }).collect()
  }).collect();

  let mut max: Vec<i8> = vec![-1; height.first().unwrap().len()];
  let mut min_height_down = height.clone();
  min_height_down.reverse();
  min_height_down = min_height_down.iter().map(|r| {
    let prev = max.clone();
    r.iter().zip(prev).enumerate().map(|(i, (c, p))| {
      max[i] = cmp::max(max[i], *c);
      p
    }).collect()
  }).collect();
  min_height_down.reverse();

  let visible = height.iter().zip(min_height_left).zip(min_height_right).zip(min_height_up).zip(min_height_down)
      .map(|((((h, l), r), u), d)| {
        h.iter().zip(l).zip(r).zip(u).zip(d)
            .map(|((((hh, ll), rr), uu), dd)| {
          (hh > &ll) || hh > &rr || hh > &uu || hh > &dd
        }).collect()
      }).collect();
  return visible;

}

fn parse(lines: &Vec<String>) -> Vec<Vec<i8>> {
  let grid = lines.iter().map(|x|
    x.chars().map(|y| (y as u8 - b'0') as i8).collect()
  ).collect();
  return grid;
}