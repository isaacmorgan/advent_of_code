use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashSet;
use crate::tools;
static FNAME: &str = "./input/2022/2022-12-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let (map, start, end) = parse(lines);
  let sp = find_shortest_path(&map, &start, &end);
  println!("Part 1 shortest path: {sp}");
}

fn part2(lines: &Vec<String>) {
  let (map, start, end) = parse(lines);
  let loc = find_loc_by_depth(&map, 'a');
  let shortest_path = loc.iter()
      .map(|l| find_shortest_path(&map, l, &end))
      .min().unwrap();
  println!("Part 2 shortest path: {shortest_path}");
}

fn find_loc_by_depth(map: &Vec<Vec<char>>, depth: char) -> Vec<(usize, usize)> {
  let mut loc = Vec::new();
  for (i, r) in map.iter().enumerate() {
    for (j, c) in r.iter().enumerate() {
      if *c == depth {
        loc.push((i, j));
      }
    }
  }
  return loc;
}

fn find_shortest_path(map: &Vec<Vec<char>>, start: &(usize, usize), end: &(usize, usize)) -> u32 {
  let mut paths: BinaryHeap<Reverse<(u32, char, usize, usize)>> = BinaryHeap::new();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();

  // Find bounds
  let max_i = map.len();
  let max_j = map.first().unwrap().len();

  // Begin search at Start
  paths.push(Reverse((0, 'a', start.0, start.1)));
  visited.insert(*start);

  // Path finding
  let mut shortest_path = u32::MAX;
  'outer:
  while !paths.is_empty() {
    let p = paths.pop().unwrap().0;
    for dij in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
      let (i, j) = (p.2 as i32 + dij.0, p.3 as i32 + dij.1);
      if i < 0 || j < 0 || i >= max_i as i32 || j >= max_j as i32 {
        continue;
      }
      let i = i as usize;
      let j = j as usize;

      if visited.contains(&(i, j)) {
        continue;
      }

      if map[i][j] as u32 <= p.1 as u32 + 1 {
        if i == end.0 && j == end.1 {
          shortest_path = p.0 + 1;
          break 'outer;
        }
        paths.push(Reverse((p.0 + 1, map[i][j], i, j)));
        visited.insert((i, j));
      }
    }
  }

  return shortest_path;
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
  let mut map: Vec<Vec<char>> = Vec::new();
  for ln in lines {
    map.push(ln.chars().collect());
  }

  // Find start
  let mut start= (0, 0);
  'outer:
  for (i, r) in map.iter().enumerate() {
    for (j, c) in r.iter().enumerate() {
      if *c == 'S' {
        start = (i, j);
        break 'outer;
      }
    }
  }

  // Find start
  let mut end= (0, 0);
  'outer:
  for (i, r) in map.iter().enumerate() {
    for (j, c) in r.iter().enumerate() {
      if *c == 'E' {
        end = (i, j);
        break 'outer;
      }
    }
  }

  map[start.0][start.1] = 'a';
  map[end.0][end.1] = 'z';
  return (map, start, end);
}
