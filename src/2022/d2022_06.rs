use std::str::Chars;
use itertools::Itertools;
use crate::tools;
static FNAME: &str = "./input/2022/2022-06-01.txt";

pub fn main() {
    let input = tools::load(FNAME);
    part1(&input);
    part2(&input);
}

fn part1(lines: &Vec<String>) {
    for ln in lines {
        let ans = find_marker(ln.as_bytes(), 4);
        println!("Part 1 Marker: {ans}");
    }
}

fn part2(lines: &Vec<String>) {
    for ln in lines {
        let ans = find_marker(ln.as_bytes(), 14);
        println!("Part 2 Marker: {ans}");
    }
}

fn find_marker(line: &[u8], n: usize) -> usize {
    let mut p0: usize = 0;
    let mut p1: usize = n;
    let mut done = false;
    let mut pass = false;
    while !done {
        pass = false;
        'pass:
        for i in p0..p1 {
            for j in (i+1)..p1 {
                //println!("Check {} and {}", line[i] as char, line[j] as char);
                if line[i] == line[j] {
                    //println!("Match");
                    pass = true;
                    break 'pass;
                }
            }
        }
        if pass {
            p0 += 1;
            p1 += 1;
        } else {
            done = true;
        }
    }
    return p1;
}
