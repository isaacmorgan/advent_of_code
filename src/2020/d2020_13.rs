use crate::tools;
use itertools::{Itertools, enumerate};
use std::collections::HashMap;

static FNAME: &str = "./input/2020/2020-13.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (t, bus, _) = load();
    println!("{} {:?}", t, bus);
    let mut min_w = i64::max_value();
    let mut min_b = 0;
    for b in bus {
        let w = b - t.rem_euclid(b);
        println!("{}: wait {}", b, w);
        if w < min_w {
            min_w = w;
            min_b = b;
        }
    }
    
    println!("Min wait score: {}", min_b * min_w);
}

fn part2() {
    let (_, bus, bus_delay) = load();
    println!("{:?}", bus);
    println!("{:?}", bus_delay);
    // Check next value until valid number is found.
    // Skip by multiples to speed up, since they're all primes it's fine.
    let mut start = 0;
    let mut mul = 1;
    for (b,d) in &bus_delay {
        loop {
            if (start + d).rem_euclid(*b) == 0 {
                mul *= b;
                break;
            }
            start += mul;
        }
    }
    println!("Time: {}", start);
}

fn load() -> (i64, Vec<i64>, HashMap<i64, i64>) {
    let input = tools::load(FNAME);
    let t = input.first().unwrap().parse().unwrap();
    
    let busses = input.get(1).unwrap().split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect_vec();
    
    let mut bus_delay = HashMap::new();
    let list = input.get(1).unwrap().split(",").collect_vec();
    for (i, s) in enumerate(list) {
        if s != "x" {
          bus_delay.insert(s.parse().unwrap(), i as i64);
        } 
    }
    
    return (t, busses, bus_delay);
}