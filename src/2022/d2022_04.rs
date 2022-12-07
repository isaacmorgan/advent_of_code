use std::collections::HashSet;
use std::ops::{Range, RangeFrom, RangeInclusive};
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-04-01.txt";

pub fn main() {
    let input = tools::load(FNAME);
    part1(&input);
    part2(&input);
}

fn part1(lines: &Vec<String>) {
    let mut count = 0;
    let ranges = parse(lines);
    for (a,b) in ranges {
        if full_overlap(&a, &b) {
            count += 1;
        }
    }
    println!("Total complete overlap: {count}");
}

fn part2(lines: &Vec<String>) {
    let mut count = 0;
    let ranges = parse(lines);
    for (a,b) in ranges {
        if any_overlap(&a, &b) {
            count += 1;
        }
    }
    println!("Total overlap: {count}");
}

fn parse(lines: &Vec<String>) -> Vec<(RangeInclusive<i32>,RangeInclusive<i32>)> {
    lines.iter().map(
        |x| x.split(',').map(
            |y| {
                let z:Vec<i32> = y.split('-').map(|t| t.parse::<i32>().unwrap()).collect();
                *z.first().unwrap()..=*z.last().unwrap()
            }
        ).collect_tuple().unwrap()
    ).collect()
}

fn full_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    (a.start() <= b.start() && a.end() >= b.end())
        || (b.start() <= a.start() && b.end() >= a.end())
}

fn any_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    !(a.start() > b.end() || a.end() < b.start())
}
