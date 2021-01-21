use crate::tools;
use std::collections::HashMap;
use itertools::{Itertools, zip, enumerate};

static FNAME: &str = "./input/2020/2020-14.txt";

struct Computer {
    mask: String,
    mask_1: u64,
    mask_0: u64,
    mem: HashMap<u64, u64>,
}

#[derive(Debug)]
struct Command {
    loc: Vec<char>,
    val: u64,
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = load();
    let mut comp = Computer{
        mask: "".to_string(),
        mask_1: 0,
        mask_0: 0,
        mem: Default::default()
    };
    for line in lines {
        step(&line, &mut comp);
    }
    println!("{:?}", comp.mem);
    
    let sum: u64 = comp.mem.values().into_iter().sum();
    println!("Values Sum: {}", sum);
}

fn part2() {
    // Translate into list of commands
    let lines = load();
    let commands = input_to_commands(&lines);
    let mut map: HashMap<u64, u64> = HashMap::new();
    for c in &commands {
        // println!("{:?}", c);
        let a = mask_to_int(&c.loc);
        // println!("{:?}", a);
        for b in a {
            map.insert(b, c.val);
        }
    }
    let s = map.values().into_iter().sum::<u64>();
    println!("Sum: {}", s);
    // println!("{:?}", map);
}

fn load() -> Vec<String> {
    tools::load(FNAME)
}

fn mask_to_int(mask: &Vec<char>) -> Vec<u64> {
    let mut n = mask.len();
    let mut base = 0;
    let mut inds: Vec<u32> = Vec::new();
    for i in 0..n {
        if mask[i] == '1' {
            base += 2_u64.pow(35 - i as u32)
        }
        if mask[i] == 'X' {
            inds.push(35 - i as u32);
        }
    }
    // println!("Mask: {:?}", mask);
    // println!("base: {:?}", base);
    // println!("inds: {:?}", inds);
    let mut out = combine(&inds[0..]);
    out.iter().map(|x| x | x + base).collect_vec()
}

fn combine(inds: &[u32]) -> Vec<u64> {
    // println!("{:?}", inds);
    if inds.is_empty() {
        return vec![0];
    }
    let a = 2_u64.pow(inds[0]);
    let rest = combine(&inds[1..]);
    let mut out = Vec::new();
    out.push(a);
    for r in &rest {
        out.push(a+r);
    }
    out.extend(rest);
    // println!("out: {:?}", out);
    return out;
}

/*fn split_overlap(old: &Vec<char>, new: &Vec<char>) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut out_old = Vec::new();
    let mut out_new = Vec::new();
    let mut inds = Vec::new();
    for (i, (o, n)) in enumerate(zip(old, new)) {
        if o == &'X' && n != &'X' {
            // println!("{}", i);
            inds.push(i);
        }
    }
    for i in 0..2_u32.pow(inds.len() as u32) {
        let tmp = old.clone();
        
    }
    return (out_old, out_new);
}*/

fn is_overlap(old: &Vec<char>, new: &Vec<char>) -> bool {
    for (o, n) in zip(old, new) {
        if (o == &'1' && n == &'0') || (o == &'0' && n == &'1') {
            return false;
        }
    }
    return true;
}

fn input_to_commands(input: &Vec<String>) -> Vec<Command> {
    let re_mem = regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let re_mask = regex::Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mut mask = vec!['0';36];
    let mut cmds = Vec::new();
    for line in input {
        if line.starts_with("mem") {
            let mut cap = re_mem.captures(line).unwrap();
            let a = cap[1].parse::<u64>().unwrap();
            let b = cap[2].parse::<u64>().unwrap();
            cmds.push(Command{ loc: mask_v2(a, &mask), val: b });
        } else if line.starts_with("mask") {
            let mut cap = re_mask.captures(line).unwrap();
            mask = cap[1].chars().collect_vec();
        }
    }
    return cmds;
}

fn mask_v2(x: u64, mask: &Vec<char>) -> Vec<char> {
    let mut out = Vec::new();
    let n = &format!("{:036b}", x).chars().collect_vec();
    for (v, m) in zip(n, mask) {
        out.push (match m {
            'X' => 'X',
            '0' => *v,
            '1' => '1',
            _ => '?',
        }
        );
    }
    return out;
}

fn step(line: &str, comp: &mut Computer) {
    let re_mem = regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let re_mask = regex::Regex::new(r"^mask = ([01X]{36})$").unwrap();
    if line.starts_with("mem") {
        let mut cap = re_mem.captures(line).unwrap();
        let a = cap[1].parse::<u64>().unwrap();
        let b = cap[2].parse::<u64>().unwrap();
        comp.mem.insert(a, mask(b, &comp));
    } else if line.starts_with("mask") {
        let mut cap = re_mask.captures(line).unwrap();
        let mask = cap[1].to_string();
        let mut mask_0: u64 = u64::max_value();
        let mut mask_1: u64 = 0;
        let mut i = 36;
        for c in mask.chars() {
            i -= 1;
            match c {
                '0' => mask_0 -= 2_u64.pow(i),
                '1' => mask_1 += 2_u64.pow(i),
                _ => (),
            }
        }
        comp.mask = mask;
        comp.mask_0 = mask_0;
        comp.mask_1 = mask_1;
    }
}

fn mask(x: u64, comp: &Computer) -> u64 {
    let mut y = x;
    y = y & comp.mask_0;
    y = y | comp.mask_1;
    return y;
}
