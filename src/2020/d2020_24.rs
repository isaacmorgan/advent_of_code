use crate::tools;
use itertools::{Itertools, zip};
use std::collections::HashSet;

static FNAME: &str = "./input/2020/2020-24.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let input = load();
    let mut tiles = HashSet::new();
    for s in &input {
        let id = id_tile(&s);
        if tiles.contains(&id) {
            tiles.remove(&id);
        } else {
            tiles.insert(id);
        }
    }
    println!("# black tiles: {}", tiles.len());
}

fn part2() {
    let input = load();
    let mut tiles = HashSet::new();
    for s in &input {
        let id = id_tile(&s);
        if tiles.contains(&id) {
            tiles.remove(&id);
        } else {
            tiles.insert(id);
        }
    }
    for _ in 0..100 {
        tiles = step(&tiles);
    }
    println!("# black tiles: {}", tiles.len());
}

fn step(black: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let check_set = get_all_neighbors(black);
    let mut next_state = black.clone(); 
    for (x, y) in check_set {
        let n = count_neighbors(x, y, black);
        if black.contains(&(x, y)) {
            if n == 0 || n > 2 {
                next_state.remove(&(x, y));
            }
        } else {
            if n == 2 {
                next_state.insert((x, y));
            }
        }
    }
    return next_state;
}

fn count_neighbors(x: i32, y: i32, black: &HashSet<(i32, i32)>) -> i32 {
    let mut cnt = 0;
    for (dx, dy) in zip(&[1, 0, -1, -1, 0, 1], &[0, 1, 1, 0, -1, -1]) {
        if black.contains(&(x + dx, y + dy)) {
            cnt += 1;
        }
    }
    return cnt;
}

fn get_all_neighbors(black: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    set.extend(black);
    for (x, y) in black {
        for (dx, dy) in zip(&[1, 0, -1, -1, 0, 1], &[0, 1, 1, 0, -1, -1]) {
            set.insert((x + dx, y + dy));
        }
    }
    return set;
}

fn id_tile(input: &str) -> (i32, i32) {
    // e, w are +- x
    // ne, sw are +- y
    // se, nw are +x-y, -x+y
    let v = input.chars().collect_vec();
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    loop {
        if i >= v.len() {
            break;
        }
        match v[i] {
            'n' => {
                i += 1;
                match v[i] {
                    'e' => y += 1,
                    'w' => {
                        x -= 1;
                        y += 1;
                    }
                    _ => (),
                }
                i += 1;
            }
            's' => {
                i += 1;
                match v[i] {
                    'e' => {
                        x += 1;
                        y -= 1;
                    }
                    'w' => y -= 1,
                    _ => (),
                }
                i += 1;
            }
            'e' => {
                x += 1;
                i += 1;
            }
            'w' => {
                x -= 1;
                i += 1;
            }
            _ => (),
        }
    }
    return (x, y);
}

fn load() -> Vec<String> {
    tools::load(FNAME)
}