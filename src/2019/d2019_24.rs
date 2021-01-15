use std::collections::{HashMap, HashSet};

const FNAME: &str = "./input/2019/2019-24.txt";
const FNAME0: &str = "./input/2019/2019-24-0.txt";
const FNAME1: &str = "./input/2019/2019-24-1.txt";
pub fn main() {
  part_1();
  part_2();
}

fn part_2() {
  let (mut map, size) = load(FNAME);
  let dmap = build_dmap(&size);
  println!("{:?}", dmap);
  //let mut bio_set = HashSet::new();
}

fn part_1() {
  let (mut map, size) = load(FNAME);
  let mut bio_set = HashSet::new();
  let mut bio = 0;
  loop {
    bio = calc_biodiversity(&map, &size);
    if bio_set.contains(&bio) {
      break;
    } else {
      bio_set.insert(bio);
    }
    step(&mut map, &size);
  }
  println!("Biodiversity Score: {}", bio);
}

fn build_dmap(size: &(i32, i32)) -> HashMap<(i32, i32), Vec<(i32, i32, i32)>>{
  let mut dmap = HashMap::new();
  let mx = &size.0/2+1;
  let my = &size.1/2+1;
  for x in 0..size.0{
    for y in 0..size.1 {
      let mut n = Vec::new();
      if x > 0 { n.push((x-1, 0, 0))}
      if x < &size.0 - 1 { n.push((x+1, 0, 0))}
      if y > 0 { n.push((0, y-1, 0))}
      if y < &size.0 - 1 { n.push((0, y+1, 0))}
      if x == 0 { n.push((mx-1, my, 1)); }
      if x == &size.0 - 1 { n.push((mx+1, my, 1)); }
      if y == 0 { n.push((mx, my-1, 1)); }
      if y == &size.1 - 1 { n.push((mx, my+1, 1)); }
      if x == mx - 1 && y == my {
        for yy in 0..size.1 { n.push((0, yy, -1)); }
      }
      if x == mx + 1 && y == my {
        for yy in 0..size.1 { n.push((&size.0 - 1, yy, -1)); }
      }
      if x == mx && y == my - 1 {
        for xx in 0..size.0 { n.push((xx, 0, -1)); }
      }
      if x == mx && y == my + 1 {
        for xx in 0..size.0 { n.push((xx, &size.1 - 1, -1)); }
      }
      dmap.insert((x, y), n);
    }
  }
  return dmap;
}

fn step(map: &mut HashSet<(i32, i32, i32)>, size: &(i32, i32) ) {
  let mut rem_list = Vec::new();
  let mut add_list = Vec::new();
  // for each in size
  for x in 0..size.0 {
    for y in 0..size.1 {
      // count neighbors
      let mut cnt = 0;
      for dxy in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        if map.contains(&(x + dxy.0, y + dxy.1, 0)) {
          cnt += 1;
        }
      }
      if map.contains(&(x, y, 0)) {
        if cnt != 1 {
          rem_list.push((x,y, 0));
        }
      } else {
        if cnt == 1 || cnt == 2 {
          add_list.push((x, y,0));
        }
      }
    }
  }
  for m in &rem_list {
    map.remove(m);
  }
  for m in &add_list {
    map.insert(*m);
  }
}

fn calc_biodiversity(map: &HashSet<(i32, i32, i32)>, size: &(i32, i32)) -> i32 {
  let mut bio = 0;
  let two: i32 = 2;
  for m in map {
    bio += two.pow((m.1*&size.1 + m.0) as u32);
  }
  return bio;
}

fn print_map(map: &HashSet<(i32, i32, i32)>, size: &(i32, i32)) {
  let zmax = map.iter().map(|x| x.2).max();
  let zmin = map.iter().map(|x| x.2).min().unwrap();
  for y in 0..size.1 {
    for x in 0..size.0 {
      if map.contains(&(x, y, 0)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!();
}

fn load(fname: &str) -> (HashSet<(i32, i32, i32)>, (i32, i32)) {
  let mut map = HashSet::new();
  let lines = std::fs::read_to_string(fname).unwrap();
  let mut x = 0;
  let mut y = 0;
  let mut max_x = 0;
  let mut max_y = 0;
  for l in lines.lines() {
    for c in l.chars() {
      if c == '#' {
        map.insert((x, y, 0));
      }
      x += 1;
    }
    max_x = x;
    x = 0;
    y += 1;
  }
  max_y = y;
  return (map, (max_x, max_y));
}
