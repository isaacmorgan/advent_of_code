use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};
use crate::tools;
static FNAME: &str = "./input/2022/2022-19-01.txt";
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct State {
  time: i32,
  robots: [i32; 4],
  minerals: [i32; 4],
  history: Vec<(i32, usize)>,
}

struct Blueprint {
  id: i32,
  robots: Vec<Robot>,
}

struct Robot {
  mineral: usize,
  cost: [i32; 4],
}

pub fn main() {
  let input = tools::load(FNAME);
  part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let n = 24;
  let blueprints = parse(&lines);
  let mut quality_sum = 0;
  for bp in blueprints.iter() {
    let score = sim_blueprint_new(&bp, n);
    let quality = score * bp.id;
    quality_sum += quality;
    println!("Blueprint {} score {score} quality {quality}", bp.id);
  }
  println!("Part 1 Quality Sum: {quality_sum}\n");
}

fn part2(lines: &Vec<String>) {
  let n = 32;
  let blueprints = parse(&lines);
  let mut score_prod = 1;
  for bp in blueprints[0..3].iter() {
    let score = sim_blueprint_new(&bp, n);
    score_prod *= score;
    println!("Blueprint {} score {score}", bp.id);
  }
  println!("Part 2 Score Product: {score_prod}\n");
}

fn sim_blueprint_new(bp: &Blueprint, n: i32) -> i32 {
  let mut max_geodes = 0;
  //let mut states = Vec::new();
  let mut states = BinaryHeap::new();
  //states.push((0, [1, 0, 0, 0], [0, 0, 0, 0])); // Minute, robots, materials
  let mut max_robots = [0, 0, 0, i32::MAX];
  for r in &bp.robots {
    for i in 0..4 {
      max_robots[i] = max(max_robots[i], r.cost[i]);
    }
  }

  states.push(State {
    time: 0,
    robots: [1, 0, 0, 0],
    minerals: [0, 0, 0, 0],
    history: Vec::new(),
  });

  while !states.is_empty() {
    //let state = states.remove(states.len() - 1);
    //println!("Len: {} Max: {} State Min: {}", states.len(), max_geodes, state.time);
    let state = states.pop().unwrap();
    //dbg!(&state);

    // Calculate max geodes if no further action is taken
    let t = n - state.time;
    if state.robots[GEODE] > 0 {
      let my_min_geodes = state.minerals[GEODE] + t*state.robots[GEODE];
      if my_min_geodes > max_geodes {
        max_geodes = my_min_geodes;
        //println!("Len: {} Max: {} State Min: {}", states.len(), max_geodes, state.time);
        //dbg!(&state);
      }
    }

    // Stop if time limit reached
    if state.time == n {
      continue;
    }

    // Stop if there's no hope of beating the max_geode count
    if state.minerals[GEODE] + state.robots[GEODE]*t + (t*t + t)/2 < max_geodes {
      continue;
    }

    // Propogate future states by checking against each robot type
    for robot in &bp.robots {
      if state.robots[robot.mineral] >= max_robots[robot.mineral] {
        continue
      }
      let new_states = propagate_to_next_build(&state, &robot, n);
      //dbg!(&new_states);
      for s in new_states {
        states.push(s);
      }
    }
  }

  return max_geodes;
}

fn propagate_to_next_build(state: &State, robot: &Robot, n: i32) -> Option<State> {
  // Calculate material needed
  let mut mat_needed = robot.cost;
  for i in 0..4 {
    mat_needed[i] -= state.minerals[i];
  }

  // Calculate days needed to collect material needed
  let mut days_needed = 0;
  for i in 0..4 {
    if mat_needed[i] > 0 {
      if state.robots[i] == 0 {
        return None;
      } else {
        days_needed = max(days_needed, 1 + (mat_needed[i] + state.robots[i] - 1) / state.robots[i]);
      }
    }
  }

  // Wait until material is collected, then build robot and return state
  if state.time + days_needed > n {
    return None;
  }

  // Minimum time to build robot is still 1 minute
  if days_needed == 0 {
    days_needed = 1;
  }

  let mut s = State {
    time: state.time,
    robots: state.robots,
    minerals: state.minerals,
    history: state.history.clone(),
  };
  s.time += days_needed;
  for i in 0..4 {
    s.minerals[i] += days_needed * s.robots[i] - robot.cost[i];
  }
  s.robots[robot.mineral] += 1;
  s.history.push((s.time, robot.mineral));

  return Some(s);
}

fn parse(lines: &Vec<String>) -> Vec<Blueprint> {
  let mut blueprints = Vec::new();
  for ln in lines {
    let mut bp = Blueprint {
      id: 0,
      robots: vec![]
    };
    let (rem, ln) = ln.split_once(": ").unwrap();
    let (_, bp_id) = rem.split_once(" ").unwrap();
    bp.id = bp_id.parse().unwrap();
    for part in ln.split(". ") {
      let (r_type, rem) = parse_robot_type(part);
      let cost = parse_robot_cost(rem);
      bp.robots.push(Robot { mineral: r_type, cost: cost });
    }
    blueprints.push(bp);
  }
  return blueprints;
}

fn parse_robot_type(line: &str) -> (usize, &str) {
  let (rem_a, cost) = line.split_once(" robot costs ").unwrap();
  let r_type = match rem_a.trim_start_matches("Each ") {
    "ore" => ORE,
    "clay" => CLAY,
    "obsidian" => OBSIDIAN,
    "geode" => GEODE,
    x => panic!("Unrecognized robot type: {x}"),
  };
  return (r_type, cost);
}

fn parse_robot_cost(line: &str) -> [i32; 4] {
  let mut cost: [i32;4] = [0; 4];
  for ln in line.split(" and ") {
    for (n, t) in ln.split_once(" ") {
      let i = match t.trim_end_matches(".") {
        "ore" => ORE,
        "clay" => CLAY,
        "obsidian" => OBSIDIAN,
        "geode" => GEODE,
        x => panic!("Unrecognized robot type: {x}"),
      };
      cost[i] += n.parse::<i32>().unwrap();
    }
  }
  return cost;
}
