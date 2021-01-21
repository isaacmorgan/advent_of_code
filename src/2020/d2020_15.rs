use crate::tools;
use std::collections::{HashMap, HashSet};
use itertools::enumerate;

static FNAME: &str = "./input/2020/2020-.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    // let input = [0, 3, 6];
    let input = [14,8,16,0,1,17];
    let mut map = HashMap::new();
    for (i, &inp) in enumerate(&input) {
        map.insert(inp, i as i32);
    }
    let mut last_num: i32 = input.last().unwrap().clone();
    let mut diff = 0;
    for i in input.len()..2020 {
        map.insert(last_num, i as i32 - 1);
        last_num = diff;
        if map.contains_key(&last_num) {
            diff = i as i32 - map.get(&last_num).unwrap();
        } else {
            diff = 0;
        }
        // println!("{}: {} : {}", i, last_num, diff);
    }
    println!("{}th number: {}", 2020, last_num);
}

fn part2() {
    let input = [14,8,16,0,1,17];
    let mut map = HashMap::new();
    for (i, &inp) in enumerate(&input) {
        map.insert(inp, i as i32);
    }
    let mut last_num: i32 = input.last().unwrap().clone();
    let mut diff = 0;
    let n = 30_000_000;
    for i in input.len()..n {        
        map.insert(last_num, i as i32 - 1);
        last_num = diff;
        if map.contains_key(&last_num) {
            diff = i as i32 - map.get(&last_num).unwrap();
        } else {
            diff = 0;
        }
        // println!("{}: {} : {}", i, last_num, diff);
    }
    println!("{}th number: {}", n, last_num);
}

fn load() {
    let input = tools::load(FNAME);
}