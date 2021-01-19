use crate::tools;
use std::fmt;
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-11.txt";

struct Map {
    val: Vec<State>,
    x: usize,
    y: usize,
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum State {
    Floor,
    Chair,
    Occupied,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> &State {
        &self.val[y * self.x + x]
    }

    fn set(&mut self, x: usize, y: usize, s: State) {
        self.val[y * self.x + x] = s;
    }

    fn print(&self) {
        for y in 0..self.y {
            let q: String = self.val[y * self.x..(y + 1) * self.x].iter().map(|x| {
                match x {
                    State::Floor => '.',
                    State::Chair => 'L',
                    State::Occupied => '#',
                }
            }).collect();
            println!("{}", q);
        }
        println!();
    }

    fn count(&self) -> usize {
        self.val.iter().filter(|x| *x == &State::Occupied).count()
    }
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let mut map = load();
    // map.print();
    while (step(&mut map)) {};
    // map.print();
    println!("Total Occupied: {}", map.count());
}

fn part2() {
    let mut map = load();
    // map.print();
    while(step_sight(&mut map)) {};
    println!("Total Occupied: {}", map.count());
}

// Return true if change
fn step_sight(map: &mut Map) -> bool {
    let mut flip = Vec::new();
    for x in 0..map.x {
        for y in 0..map.y {
            let s = map.get(x, y);
            if s == &State::Floor {
                continue;
            }
            let n = count_occupied_sight(&map, x, y);
            if s == &State::Chair && n == 0 {
                flip.push((x, y, State::Occupied));
            } else if s == &State::Occupied && n >= 5 {
                flip.push((x, y, State::Chair));
            }
        }
    }
    let did_change = !flip.is_empty();
    for (x, y, s) in flip {
        map.set(x, y, s);
    }
    return did_change;
}

// Return true if change
fn step(map: &mut Map) -> bool {
    let mut flip = Vec::new();
    for x in 0..map.x {
        for y in 0..map.y {
            let s = map.get(x, y);
            if s == &State::Floor {
                continue;
            }
            let n = count_occupied_neighbors(&map, x, y);
            if s == &State::Chair && n == 0 {
                flip.push((x, y, State::Occupied));
            } else if s == &State::Occupied && n >= 4 {
                flip.push((x, y, State::Chair));
            }
        }
    }
    let did_change = !flip.is_empty();
    for (x, y, s) in flip {
        map.set(x, y, s);
    }
    return did_change;
}

fn count_occupied_sight(map: &Map, x: usize, y: usize) -> usize {
    let mut cnt = 0;
    for dx in &[-1, 0, 1] {
        for dy in &[-1, 0, 1] {
            if dx == &0 && dy == &0 {
                continue;
            }
            for n in 1..map.x {
                let xx = x as i32 + dx * n as i32;
                let yy = y as i32 + dy * n as i32;
                if !((xx >= 0) && (xx < map.x as i32) && (yy >= 0) && (yy < map.y as i32)) {
                    break;
                }
                if map.get(xx as usize, yy as usize) == &State::Occupied {
                    cnt += 1;
                    break;
                }
                if map.get(xx as usize, yy as usize) == &State::Chair {
                    break;
                }
            }
        }
    }
    // println!("X: {} y: {} cnt: {}", x, y, cnt);
    return cnt;
}

fn count_occupied_neighbors(map: &Map, x: usize, y: usize) -> usize {
    let mut cnt = 0;
    for dx in &[-1, 0, 1] {
        for dy in &[-1, 0, 1] {
            if dx == &0 && dy == &0 {
                continue;
            }
            if (x as i32 + dx >= 0) && (x as i32 + dx < map.x as i32) && (y as i32 + dy >= 0) && (y as i32 + dy < map.y as i32)
                && map.get((x as i32 + dx) as usize, (y as i32 + dy) as usize) == &State::Occupied {
                cnt += 1;
            }
        }
    }
    return cnt;
}

fn load() -> Map {
    let input = tools::load(FNAME);
    let y = input.len();
    let x = input[0].len();
    let mut val = Vec::new();
    for line in &input {
        for c in line.chars() {
            if let Some(i) = match c {
                '#' => Some(State::Occupied),
                '.' => Some(State::Floor),
                'L' => Some(State::Chair),
                _ => None,
            } {
                val.push(i);
            }
        }
    }
    Map {
        val,
        x,
        y,
    }
}