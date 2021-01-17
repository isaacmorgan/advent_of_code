use crate::tools;
use itertools::Itertools;

const FNAME: &str = "./input/2020/2020-02.txt";

pub fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    char: char,
    pass: Vec<char>,
}

fn part1() {
    let input = load(FNAME);
    let mut cnt = 0;
    for p in &input {
        let n = p.pass.iter().filter(|&c| *c == p.char).count();
        if n as i32 >= p.min && n as i32 <= p.max {
            cnt += 1;
        }
    }
    println!("There are {} valid passwords.", cnt);
}


fn part2() {
    let input = load(FNAME);
    let mut cnt = 0;
    for p in &input {
        if (p.pass[(p.min-1) as usize] == p.char) != (p.pass[(p.max-1) as usize] == p.char) {
            cnt += 1;
        }
    }
    println!("There are {} valid passwords.", cnt);

}

fn load(fname: &str) -> Vec<Password> {
    let input = tools::load(fname);
    let mut pass = Vec::new();
    for l in input {
        let mut p = l.split_whitespace();
        let q = p.next().unwrap();
        let mut r = q.split("-");
        let a = r.next().unwrap().parse::<i32>().unwrap();
        let b = r.next().unwrap().parse::<i32>().unwrap();
        let c = p.next().unwrap().chars().next().unwrap();
        let d = p.next().unwrap().chars().collect_vec();
        pass.push( Password {
            min: a, max: b, char: c, pass: d,
        });
    }
    return pass;
}