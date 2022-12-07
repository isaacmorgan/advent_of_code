use std::fs;
use std::hash;
use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Replacer;
use std::collections::{HashMap, HashSet};

const GROUND: i32 = 0;
const TREE: i32 = 1;
const LUMBER: i32 = 2;

const NX: usize = 50;
const NY: usize = 50;

const FNAME: &str = "./input/2018/2018-18.txt";

pub fn main() {
    let mut map = load();
    let mut history = HashMap::new();
    let mut resources = HashMap::new();

    println!("After {} minutes:", 0);
    print_map(&map);
    let first_hash = map_hash(&map);
    let mut first_hash_in_loop = 0;
    let mut old_hash = map_hash(&map);
    let mut new_hash = old_hash;
    resources.insert(new_hash, count_resources(&map));
    let mut cnt = 0;
    for t in 1..=600 {
        cnt += 1;
        if history.contains_key(&new_hash) {
            if first_hash_in_loop == 0 {
                first_hash_in_loop = new_hash;
            }
            //old_hash = new_hash;
            //new_hash = *history.get(&new_hash).unwrap();
            //break;
            cnt -= 1;
        }
        step(&mut map);
        old_hash = new_hash;
        new_hash = map_hash(&map);
        history.insert(old_hash, new_hash);
        resources.insert(new_hash, count_resources(&map));
        println!("After {} minutes {}", t, count_resources(&map));
    }
    cnt = cnt;
    println!("First repeat at {}", cnt);
    let mut c = 0;
    new_hash = first_hash_in_loop;
    loop {
        println!("{}: {}: {}", c, new_hash, resources.get(&new_hash).unwrap());
        c += 1;
        new_hash = *history.get(&new_hash).unwrap();
        if new_hash == first_hash_in_loop {
            break;
        }
    }
    //c = c;

    println!("cnt: {} c: {} Answer at: {}", cnt, c, (1_000_000_000 - cnt)%(c));
    println!("Total resource value: {}", resources.get(&new_hash).unwrap());
}

fn map_hash(map: &[[i32; NY]; NX]) -> u64 {
    let mut h = DefaultHasher::new();
    let mut s = String::new();
    for y in 0..NY {
        for x in 0..NX {
            s.push_str(&map[x][y].to_string());
        }
    }
    s.hash(&mut h);
    h.finish()
}

fn count_resources(map: &[[i32; NY]; NX]) -> i32 {
    let mut n_tree = 0;
    let mut n_lumb = 0;
    for x in 0..NX {
        for y in 0..NY {
            match map[x][y] {
                TREE => n_tree += 1,
                LUMBER => n_lumb += 1,
                _ => ()
            }
        }
    }
    return n_tree * n_lumb;
}

fn step(map: &mut [[i32; NY]; NX]) {
    let mut tmp = [[0; NY]; NX];
    for x in 0..NX {
        for y in 0..NY {
            tmp[x][y] = map[x][y];
        }
    }

    for y in 0..NY {
        for x in 0..NX {
            match tmp[x][y] {
                GROUND => if count_neighbors(&tmp, x, y, TREE) >= 3 {
                    map[x][y] = TREE;
                } ,
                TREE => if count_neighbors(&tmp, x, y, LUMBER) >= 3 {
                    map[x][y] = LUMBER;
                } ,
                LUMBER => if !(count_neighbors(&tmp, x, y, TREE) >= 1 && (count_neighbors(&tmp, x, y, LUMBER) >= 1)) {
                    map[x][y] = GROUND;
                } ,
                _ => (),
            }
        }
    }
}

fn count_neighbors(map: &[[i32; NY]; NX], x0: usize, y0: usize, neighbor: i32) -> i32 {
    let mut cnt = 0;
    for x in (max(0, x0 as i32 - 1) as usize)..(min(NX, x0 + 2)) {
        for y in (max(0, y0 as i32 - 1) as usize)..(min(NY, y0 + 2)) {
            if x == x0 && y == y0 {
                continue;
            }
            if map[x][y] == neighbor {
                cnt += 1;
            }
        }
    }
    return cnt;
}

fn print_map(map: &[[i32; NY]; NX]) {
    for y in 0..NY {
        for x in 0..NX {
            print!("{}", match map[x][y] {
                GROUND => '.',
                TREE => '|',
                LUMBER => '#',
                _ => ' ',
            });
        }
        println!();
    }
    println!();
}

fn load() -> [[i32; NY]; NX] {
    let mut map = [[0; NY]; NX];
    let contents = fs::read_to_string(FNAME).unwrap();
    let mut lines = contents.split("\n");
    for y in 0..NY {
        let mut line = lines.next().unwrap().chars();
        for x in 0..NX {
            map[x][y] = match line.next().unwrap() {
                '.' => GROUND,
                '|' => TREE,
                '#' => LUMBER,
                _ => 0,
            }
        }
    }
    return map;
}
