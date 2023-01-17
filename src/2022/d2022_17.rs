use std::collections::HashSet;
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-17-01.txt";
const WIDTH: usize = 7;

pub fn main() {
  let input = tools::load(FNAME);
  let shapes: Vec<Vec<(i32, i32)>> = vec![
    vec![(0, 0), (0, 1), (0, 2), (0, 3)],
    vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)],
    vec![(0, 0), (1, 0), (2, 0), (3, 0)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
  ];
  part1(&input, &shapes);
  part2(&input, &shapes);
}

fn part1(lines: &Vec<String>, shapes: &Vec<Vec<(i32, i32)>>) {
  let cmds = parse(lines);
  let mut map = HashSet::new();
  let mut peak = 0;
  for i in 0..WIDTH {
    map.insert((0, i as i32));
  }
  let mut ind: usize = shapes.len() - 1;
  let mut ci: usize = 0;
  for i in 0..2022 {
    ind = (ind+1)%shapes.len();
    let mut rock = spawn_rock(&map, &shapes.get(ind).unwrap());
    while step(&mut map, &mut rock, cmds.get(ci%cmds.len()).unwrap(), &mut peak) {
      ci += 1;
    };
    ci += 1;
  }

  let height = peak.abs();
  println!("Part 1 tower height: {height}");
}

fn part2(lines: &Vec<String>, shapes: &Vec<Vec<(i32, i32)>>) {
  let cmds = parse(lines);
  let mut map = HashSet::new();
  let mut peak = 0;
  for i in 0..WIDTH {
    map.insert((0, i as i32));
  }
  let mut ind: usize = shapes.len() - 1;
  let mut ci: usize = 0;
  let mut height = Vec::new();
  for i in 0..15022 {
    ind = (ind+1)%shapes.len();
    let mut rock = spawn_rock(&map, &shapes.get(ind).unwrap());
    while step(&mut map, &mut rock, cmds.get(ci%cmds.len()).unwrap(), &mut peak) {
      ci += 1;
    };
    height.push(peak.clone().abs());
    ci += 1;
  }

  find_pattern(&height, cmds.len() as i32);

}

fn find_pattern(list: &Vec<i32>, max: i32) {
  let s = 3000;
  'outer:
  for d in 1..2000 {
    let delta = list[s + d] - list[s];
    for i in 2..6 {
      if list[s + d*i] - list[s + d*(i-1)] != delta {
        continue 'outer;
      }
    }
    println!("Length: {d} Delta: {delta} Rem: {}",  (1_000_000_000_000 - (((1_000_000_000_000 - s as u64 - 1)/d as u64) * d as u64) as usize));
    let n_cycles = (1_000_000_000_000 - s as u64 - 1)/d as u64;
    let n_rem = 1_000_000_000_000 - s as u64 - 1 - n_cycles * d as u64;
    let total = list[s + n_rem as usize] as u64 + n_cycles * delta as u64;
    dbg!(n_rem, total);
    break;
  }
}

fn hash_line(line: &Vec<i32>) -> i32 {
  let mut s = 0;
  for l in line {
    s += match l {
      0 => 1,
      1 => 2,
      2 => 4,
      3 => 8,
      4 => 16,
      5 => 32,
      6 => 64,
      _ => panic!("Unexpected row: {l}"),
    };
  }
  return s;
}

fn print_map(map: &HashSet<(i32, i32)>) {
  let min_y = *map.iter().map(|(y,x)| y).min().unwrap();
  for y in min_y..=0 {
    for x in 0..WIDTH as i32 {
      if map.contains(&(y, x)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }

  println!();
}

fn step(map: &mut HashSet<(i32, i32)>, rock: &mut Vec<(i32,i32)>, cmd: &char, peak: &mut i32) -> bool {
  let mut t_rock = translate(&rock, cmd);
  if check(&map, &t_rock) {
    rock.clear();
    t_rock.clone_into(rock);
  }

  t_rock = translate(&rock, &'v');
  if check(&map, &t_rock) {
    rock.clear();
    t_rock.clone_into(rock);
  } else {
    for r in rock {
      map.insert(*r);
      if &r.0 < peak {
        *peak = r.0;
      }
    }
    return false;
  }
  return true;
}

fn spawn_rock(map: &HashSet<(i32, i32)>, shape: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
  let dy = map.iter().map(|(y, x)| y).min().unwrap()
      - shape.iter().map(|(y, x)| y).max().unwrap()
      - 4;
  let dx = 2;
  let mut rock = shape.clone();
  for r in rock.iter_mut() {
    r.0 += dy;
    r.1 += dx;
  }
  return rock;
}

fn translate(rock: &Vec<(i32, i32)>, dir: &char) -> Vec<(i32, i32)> {
  let mut new_rock = rock.clone();
  match dir {
    '<' => new_rock.iter_mut().for_each(|x| x.1 -= 1),
    '>' => new_rock.iter_mut().for_each(|x| x.1 += 1),
    'v' => new_rock.iter_mut().for_each(|x| x.0 += 1),
    _ => panic!("Unrecognized direction: {dir}"),
  }
  return new_rock;
}

fn check(map: &HashSet<(i32, i32)>, pos: &Vec<(i32, i32)>) -> bool {
  for p in pos {
    if map.contains(&p)
      || p.1 < 0
      || p.1 >= WIDTH as i32 {
      return false;
    }
  }
  return true;
}

fn parse(lines: &Vec<String>) -> Vec<char> {
  lines.first().unwrap().chars().collect()
}
