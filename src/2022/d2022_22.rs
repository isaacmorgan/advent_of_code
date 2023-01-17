use std::collections::HashMap;
use crate::tools;
static FNAME: &str = "./input/2022/2022-22-01.txt";

#[derive(Debug)]
enum Command {
  Walk(i32),
  Turn(char),
}

pub fn main() {
  let input = tools::load(FNAME);
  //part1(&input);
  part2(&input);
}

fn part1(lines: &Vec<String>) {
  let (map, commands) = parse(lines);
  let nr = map.keys().map(|x| x.0).max().unwrap();
  let nc = map.keys().map(|x| x.1).max().unwrap();

  let mut row = map.keys().map(|x| x.0).min().unwrap();
  let mut col = map.keys().filter(|x| x.0 == row).map(|x| x.1).min().unwrap();
  let mut face = 'E';

  for cmd in &commands {
    match cmd {
      Command::Walk(x) => {(row, col, _) = walk(&map, &(row, col, face), x);},
      Command::Turn(x) => {face = turn(&face, x);},
    }
    //dbg!(&cmd);
    println!("r: {row} c: {col} f: {face}");
  }

  let sum = 1000*row + 4*col + match face {
    'N' => 3,
    'S' => 1,
    'E' => 0,
    'W' => 2,
    _ => panic!("Unrecognized face: {face}"),
  };

  println!("Part 1 Final row: {row} col: {col} face {face} sum: {sum}");
}

fn part2(lines: &Vec<String>) {
  let (map, commands) = parse(lines);
  let nr = map.keys().map(|x| x.0).max().unwrap();
  let nc = map.keys().map(|x| x.1).max().unwrap();

  let mut group = parse_group_full(&map, nr, nc);

  let mut row = map.keys().map(|x| x.0).min().unwrap();
  let mut col = map.keys().filter(|x| x.0 == row).map(|x| x.1).min().unwrap();
  let mut face = 'E';

  for cmd in &commands {
    match cmd {
      Command::Walk(x) => {(row, col, face) = walk_cube_full(&map, &(row, col, face), x, &group, &(nr/4));},
      Command::Turn(x) => {face = turn(&face, x);},
    }
    //dbg!(&cmd);
    //println!("r: {row} c: {col} f: {face}");
  }

  let sum = 1000*row + 4*col + match face {
    'N' => 3,
    'S' => 1,
    'E' => 0,
    'W' => 2,
    _ => panic!("Unrecognized face: {face}"),
  };

  println!("Part 2 Final row: {row} col: {col} face {face} sum: {sum}");
}

fn parse_group(map: &HashMap<(i32, i32), char>, nr: i32, nc: i32) -> HashMap<(i32, i32), i32> {
  let mut group = HashMap::new();
  for (r,c) in map.keys() {
    let g =
        if r <= &(nr / 3) { // Group 1
          1
        } else if r <= &(2 * nr / 3) { // Groups 2, 3, 4
          if c <= &(nc / 4) { // Group 2
            2
          } else if c <= &(2 * nc / 4) { // Group 3
            3
          } else { // Group 4
            4
          }
        } else { // Groups 5, 6
          if c <= &(3 * nc / 4) { // Group 5
            5
          } else { // Group 6
            6
          }
        };
    group.insert((*r, *c), g);
  }

  return group;
}

fn parse_group_full(map: &HashMap<(i32, i32), char>, nr: i32, nc: i32) -> HashMap<(i32, i32), i32> {
  let mut group = HashMap::new();
  for (r,c) in map.keys() {
    let g =
        if r <= &(nr / 4) { // Groups 1, 2
          if c <= &(2 * nc / 3) {
            1
          } else {
            2
          }
        } else if r <= &(2 * nr / 4) { // Groups 3
          3
        } else if r <= &(3 * nr / 4) { // Groups 4, 5
          if c <= &(1 * nc / 3) { // Group 5
            4
          } else { // Group 6
            5
          }
        } else {
          6
        };
    group.insert((*r, *c), g);
  }

  return group;
}

fn rotate_cube(group: &i32, face: &char) -> char {
  match (group, face) {
    (1, 'N') => 'S',
    (1, 'E') => 'W',
    (1, 'W') => 'S',
    (2, 'N') => 'S',
    (2, 'W') => 'N',
    (2, 'S') => 'N',
    (3, 'N') => 'E',
    (3, 'S') => 'E',
    (4, 'E') => 'S',
    (5, 'W') => 'N',
    (5, 'S') => 'N',
    (6, 'N') => 'W',
    (6, 'E') => 'W',
    (6, 'S') => 'E',
    _ => *face,
  }
}

fn rotate_cube_full(group: &i32, face: &char) -> char {
  match (group, face) {
    (1, 'N') => 'E',
    (1, 'W') => 'E',
    (2, 'E') => 'W',
    (2, 'S') => 'W',
    (3, 'E') => 'N',
    (3, 'W') => 'S',
    (4, 'N') => 'E',
    (4, 'W') => 'E',
    (5, 'E') => 'W',
    (5, 'S') => 'W',
    (6, 'E') => 'N',
    (6, 'W') => 'S',
    _ => *face,
  }
}

fn warp_cube_full(group: &i32, face: &char, dest: &(i32, i32), n: &i32) -> (i32, i32) {
  match (group, face) {
    (1, 'N') => (2*n + dest.1, 1),
    (1, 'W') => (3*n - dest.0 + 1, 1),
    (2, 'E') => (3*n - dest.0 + 1, 2*n),
    (2, 'S') => (dest.1 - n, 2*n),
    (2, 'N') => (4*n, dest.1 - 2*n),
    (3, 'E') => (1*n, dest.0 + n),
    (3, 'W') => (2*n + 1, dest.0 - n),
    (4, 'N') => (dest.1 + n, n + 1),
    (4, 'W') => (3*n - dest.0 + 1, n + 1),
    (5, 'E') => (3*n - dest.0 + 1, 3*n),
    (5, 'S') => (dest.1 + 2*n, 1*n),
    (6, 'E') => (3*n, dest.0 - 2*n),
    (6, 'W') => (1, dest.0 - 2*n),
    (6, 'S') => (1, dest.1 + 2*n),
    _ => *dest,
  }
}

fn warp_cube(group: &i32, face: &char, dest: &(i32, i32), n: &i32) -> (i32, i32) {
  match (group, face) {
    (1, 'N') => (n + 1, 3*n - dest.1 + 1),
    (1, 'E') => (3*n - dest.0 + 1, 4*n),
    (1, 'W') => (n + 1, n + dest.0),
    (2, 'N') => (1, 3*n - dest.1 + 1),
    (2, 'W') => (3*n, 4*n - dest.0 + 1),
    (2, 'S') => (3*n, 3*n - dest.1 + 1),
    (3, 'N') => (dest.1 - n, 2*n + 1),
    (3, 'S') => (4*n - dest.1 + 1, 2*n + 1),
    (4, 'E') => (2*n + 1, 5*n - dest.0 + 1),
    (5, 'W') => (2*n, 4*n - dest.0 + 1),
    (5, 'S') => (2*n, 3*n - dest.1 + 1),
    (6, 'N') => (4*n - dest.1 + 1, 3*n),
    (6, 'E') => (3*n - dest.0 + 1, 3*n),
    (6, 'S') => (5*n - dest.1 + 1, 1),
    _ => *dest,
  }
}

fn walk_cube_full(map: &HashMap<(i32, i32), char>, state: &(i32, i32, char), steps: &i32, group: &HashMap<(i32, i32), i32>, n: &i32) -> (i32, i32, char) {
  let mut state = state.clone();
  for i in 0..*steps {
    dbg!(&state);
    let dest = match state.2 {
      'N' => (state.0 - 1, state.1),
      'S' => (state.0 + 1, state.1),
      'E' => (state.0, state.1 + 1),
      'W' => (state.0, state.1 - 1),
      _ => panic!("Unrecognized face: {}", state.2),
    };
    match map.get(&dest) {
      Some('.') => { state.0 = dest.0; state.1 = dest.1; },
      Some('#') => break,
      None => {
        let g = group.get(&(state.0, state.1)).unwrap();
        let dest2 = warp_cube_full(&g, &state.2, &dest, &n);
        let face2 = rotate_cube_full(&g, &state.2);
        match map.get(&dest2) {
          Some('.') => { state.0 = dest2.0; state.1 = dest2.1; state.2 = face2; },
          Some('#') => break,
          None => panic!("Unexpected map None: {:?}", &dest2),
          _ => panic!("Unrecognized map char: {:?}", map.get(&dest2)),
        }
      },
      _ => panic!("Unrecognized map char: {:?}", map.get(&dest)),
    }
  }

  return state;
}

fn walk_cube(map: &HashMap<(i32, i32), char>, state: &(i32, i32, char), steps: &i32, group: &HashMap<(i32, i32), i32>, n: &i32) -> (i32, i32, char) {
  let mut state = state.clone();
  for i in 0..*steps {
    dbg!(&state);
    let dest = match state.2 {
      'N' => (state.0 - 1, state.1),
      'S' => (state.0 + 1, state.1),
      'E' => (state.0, state.1 + 1),
      'W' => (state.0, state.1 - 1),
      _ => panic!("Unrecognized face: {}", state.2),
    };
    match map.get(&dest) {
      Some('.') => { state.0 = dest.0; state.1 = dest.1; },
      Some('#') => break,
      None => {
        let g = group.get(&(state.0, state.1)).unwrap();
        let dest2 = warp_cube(&g, &state.2, &dest, &n);
        let face2 = rotate_cube(&g, &state.2);
        match map.get(&dest2) {
          Some('.') => { state.0 = dest2.0; state.1 = dest2.1; state.2 = face2; },
          Some('#') => break,
          None => panic!("Unexpected map None: {:?}", &dest2),
          _ => panic!("Unrecognized map char: {:?}", map.get(&dest2)),
        }
      },
      _ => panic!("Unrecognized map char: {:?}", map.get(&dest)),
    }
  }

  return state;
}

fn turn(face: &char, rotate: &char) -> char {
  match face {
    'N' => {
      match rotate {
        'L' => 'W',
        'R' => 'E',
        _ => panic!("Unexpected rotate char: {rotate}"),
      }
    },
    'S' => {
      match rotate {
        'L' => 'E',
        'R' => 'W',
        _ => panic!("Unexpected rotate char: {rotate}"),
      }
    },
    'E' => {
      match rotate {
        'L' => 'N',
        'R' => 'S',
        _ => panic!("Unexpected rotate char: {rotate}"),
      }
    },
    'W' => {
      match rotate {
        'L' => 'S',
        'R' => 'N',
        _ => panic!("Unexpected rotate char: {rotate}"),
      }
    },
    _ => panic!("Unexpected face: {face}"),
  }
}

fn walk(map: &HashMap<(i32, i32), char>, state: &(i32, i32, char), steps: &i32) -> (i32, i32, char) {
  let mut state = state.clone();
  for i in 0..*steps {
    let dest = match state.2 {
      'N' => (state.0 - 1, state.1),
      'S' => (state.0 + 1, state.1),
      'E' => (state.0, state.1 + 1),
      'W' => (state.0, state.1 - 1),
      _ => panic!("Unrecognized face: {}", state.2),
    };
    match map.get(&dest) {
      Some('.') => { state.0 = dest.0; state.1 = dest.1; },
      Some('#') => break,
      None => {
        let dest2 = match state.2 {
          'N' => (map.keys().filter(|x| x.1 == state.1).map(|x| x.0).max().unwrap(), dest.1),
          'S' => (map.keys().filter(|x| x.1 == state.1).map(|x| x.0).min().unwrap(), dest.1),
          'E' => (dest.0, map.keys().filter(|x| x.0 == state.0).map(|x| x.1).min().unwrap()),
          'W' => (dest.0, map.keys().filter(|x| x.0 == state.0).map(|x| x.1).max().unwrap()),
          _ => panic!("Unrecognized face: {}", state.2),
        };
        match map.get(&dest2) {
          Some('.') => { state.0 = dest2.0; state.1 = dest2.1; },
          Some('#') => break,
          None => panic!("Unexpected map None"),
          _ => panic!("Unrecognized map char: {:?}", map.get(&dest2)),
        }
      },
      _ => panic!("Unrecognized map char: {:?}", map.get(&dest)),
    }
  }

  return state;
}

fn parse(lines: &Vec<String>) -> (HashMap<(i32, i32), char>, Vec<Command>) {
  let mut mode = false;
  let mut command_str = "";
  let mut commands = Vec::new();
  let mut map = HashMap::new();
  let mut row = 0;
  let mut col = 0;
  for ln in lines {
    if mode {
      command_str = ln;
    } else if ln.is_empty() {
      mode = true;
      continue;
    } else {
      row += 1;
      col = 0;
      for c in ln.chars() {
        col += 1;
        match c {
          '.' => map.insert((row, col), '.'),
          '#' => map.insert((row, col), '#'),
          _ => None,
        };
      }
    }
  }

  let mut num = String::new();
  for c in command_str.chars() {
    match c {
      'R' | 'L' => {
        commands.push(Command::Walk(num.parse::<i32>().unwrap()));
        commands.push(Command::Turn(c));
        num.clear();
      },
      _ => num.push(c),
    }
  }
  if !num.is_empty() {
    commands.push(Command::Walk(num.parse::<i32>().unwrap()));
  }

  return (map, commands);
}
