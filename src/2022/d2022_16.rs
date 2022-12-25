use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::tools;
static FNAME: &str = "./input/2022/2022-16-01.txt";

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let (rate_map, dest_map) = parse(lines);

  // Get list of non-zero valves
  let mut valves: Vec<&str> = rate_map.iter().filter(|(k, v)| v > &&0).map(|(k, v)| *k).collect();
  valves.push("AA"); // Add AA since we start there.

  // Find shortest path between all non-zero valves
  let mut travel_cost = HashMap::new();
  for a in &valves {
    for b in &valves {
      if a == b {
        continue;
      }
      let cost = shortest_path(a, b, &dest_map);
      travel_cost.insert((*a,*b), cost);
    }
  }

  valves.remove(valves.len() - 1); // Remove AA.

  // Iterate all paths between non-zero valves to find maximum flow within 30 minutes
  let (max_pressure, _) = highest_pressure("AA", &travel_cost, &rate_map, &valves, 30);

  println!("Part 1 Max Pressure Release: {max_pressure}");
  //dbg!(&rate_map);
  //dbg!(&dest_map);
  //dbg!(&valves);
  //dbg!(travel_cost);

}

fn part2(lines: &Vec<String>) {
  let (rate_map, dest_map) = parse(lines);

  // Get list of non-zero valves
  let mut valves: Vec<&str> = rate_map.iter().filter(|(k, v)| v > &&0).map(|(k, v)| *k).collect();
  valves.push("AA"); // Add AA since we start there.

  // Find shortest path between all non-zero valves
  let mut travel_cost = HashMap::new();
  for a in &valves {
    for b in &valves {
      if a == b {
        continue;
      }
      let cost = shortest_path(a, b, &dest_map);
      travel_cost.insert((*a,*b), cost);
    }
  }

  valves.remove(valves.len() - 1); // Remove AA.

  // Iterate all paths between non-zero valves to find maximum flow within 30 minutes
  let (_, mut paths) = highest_pressure("AA", &travel_cost, &rate_map, &valves, 26);

  println!("cnt: {}", paths.len());

  paths.sort_by(|a,b| b.0.partial_cmp(&a.0).unwrap());

  let mut max_pressure = 0;
  for (i, a) in paths.iter().enumerate() {
    if 2*a.0 < max_pressure {
      break;
    }
    'inner:
    for b in paths.iter().skip(i + 1) {
      let sum = a.0 + b.0;
      if sum <= max_pressure {
        break;
      }

      // Break if any overlap
      for sa in &a.1 {
        for sb in &b.1 {
          if sa == sb {
            continue 'inner;
          }
        }
      }

      max_pressure = sum;
    }
  }
  println!("Part 2 Max Pressure Release: {max_pressure}");
}

fn highest_pressure<'a>(source: &str, travel_cost: &HashMap<(&str, &str), i32>, rate_map: &HashMap<&str, i32>, valves: &Vec<&'a str>, max_cost: i32) -> (i32, Vec<(i32, HashSet<&'a str>)>) {
  let mut states:Vec<(i32, &str, HashSet<&str>, i32, i32)> = Vec::new(); // Time, valve, closed valves, rate, pressure
  let mut valve_set = HashSet::new();
  let mut final_states = Vec::new();
  valves.iter().for_each(|v| {valve_set.insert(*v);});
  let mut max_pressure = 0;
  states.push((0, source, valve_set.clone(), 0, 0));
  while !states.is_empty() {
    let s = states.pop().unwrap();
    //dbg!(&s);
    if s.0 == max_cost {
      let mut vset = valve_set.clone();
      for v in s.2 {
        vset.remove(v);
      }
      final_states.push((s.4, vset));
      if s.4 > max_pressure {
        max_pressure = s.4;
        //dbg!(max_pressure);
        //dbg!(&states.len());
      }
      continue;
    }
    for d in &s.2 {
      let mut vs = s.2.clone();
      vs.remove(d);
      let mut cost = travel_cost.get(&(s.1, d)).unwrap().clone();
      if s.0 + cost + 1 <= max_cost {
        states.push((s.0 + cost + 1, d, vs, s.3 + rate_map.get(d).unwrap(), s.4 + s.3*(cost + 1)));
      }
    }
    let r = max_cost - s.0;
    states.push((max_cost, s.1, s.2, s.3, s.4 + s.3*r));
  }

  return (max_pressure, final_states);
}

fn shortest_path(source: &str, dest: &str, dest_map: &HashMap<&str, Vec<&str>>) -> i32 {
  let mut states = BinaryHeap::new();
  let mut visited = HashSet::new();
  visited.insert(source);
  states.push(Reverse((0, source)));
  loop {
    let s = states.pop().unwrap().0;
    let options = dest_map.get(s.1).unwrap();
    for o in options {
      if o == &dest {
        return s.0 + 1;
      }
      if s.0 >= 30 {
        continue;
      }
      if !visited.contains(o) {
        visited.insert(o);
        states.push(Reverse((s.0 + 1, o)));
      }
    }
  }
  return i32::MAX;
}

fn parse(lines: &Vec<String>) -> (HashMap<&str, i32>, HashMap<&str, Vec<&str>>) {
  let mut rate_map = HashMap::new();
  let mut dest_map = HashMap::new();
  for ln in lines {
    let(rem_a, rem_b) = ln.split_once(";").unwrap();
    let(rem_a, rate) = rem_a.split_once(" has flow rate=").unwrap();
    let(rem_a, source) = rem_a.split_once(" ").unwrap();
    let(rem_a, rem_b) = rem_b.split_once("valve").unwrap();
    let(rem_b, dest) = rem_b.split_once(" ").unwrap();

    let dest: Vec<&str> = dest.split(", ").collect();
    let rate: i32 = rate.parse().unwrap();

    rate_map.insert(source, rate);
    dest_map.insert(source, dest);
  }
  return (rate_map, dest_map);
}
