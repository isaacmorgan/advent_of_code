use crate::tools;
use itertools::{Itertools, zip};
use std::collections::HashMap;

static FNAME: &str = "./input/2020/2020-10.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let mut adapters = load();
    let dev = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(dev);
    adapters.sort();
    let mut diff = Vec::new();
    for (a,b) in zip(&adapters[0..adapters.len()-1], &adapters[1..]) {
        diff.push(b-a);
    }
    let n1 = diff.iter().filter(|x| *x == &1).count();
    let n3 = diff.iter().filter(|x| *x == &3).count();
    println!("{} * {} = {}", n1, n3, n1*n3);
}

fn part2() {
    let mut adapters = load();
    let dev = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(dev);
    adapters.sort();
    // for each number, sum all the ways to get to the previous numbers in range
    let mut map: HashMap<i32, i64> = HashMap::new();
    map.insert(adapters[0], 1);
    for a in &adapters[1..] {
        let n_paths = adapters.iter()
            .filter(|x| *x < a && *x >= &(a-3))
            .map(|x| map.get(x).unwrap())
            .sum::<i64>();
        map.insert(*a, n_paths);
    }
    let v = map.get(adapters.last().unwrap()).unwrap();
    println!("Max number of combinations: {}", v);
}

fn load() -> Vec<i32> {
    let input = tools::load(FNAME);
    input.iter().map(|x| x.parse().unwrap()).collect_vec()
}