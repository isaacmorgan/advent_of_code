use crate::tools;
use itertools::{Itertools};
use std::collections::HashSet;

static FNAME: &str = "./input/2020/2020-08.txt";

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Cpu {
    pc: i32,
    acc: i32,
}

#[derive(Copy, Clone)]
enum Op {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let program = load();
    let mut cpu = Cpu {
        pc: 0,
        acc: 0
    };
    
    let mut set = HashSet::new();
    set.insert(cpu.pc);
    loop {
        step(&program, &mut cpu);
        if set.contains(&cpu.pc) {
            break;
        } else {
            set.insert(cpu.pc);
        }
    }
    println!("Acc at loop: {}", cpu.acc);
}

fn part2() {
    let program = load();
    
    for i in 0..program.len() {
        match program[i] {
            Op::JMP(v) => {
                let mut pnew = Vec::new();
                pnew.clone_from(&program);
                pnew.remove(i);
                pnew.insert(i, Op::NOP(v));
                let acc = run_program(&pnew);
                if acc > 0 {
                    println!("Acc: {}", acc);
                    break;
                }
            },
            Op::NOP(v) => {
                let mut pnew = Vec::new();
                pnew.clone_from(&program);
                pnew.remove(i);
                pnew.insert(i, Op::JMP(v));
                let acc = run_program(&pnew);
                if acc > 0 {
                    println!("Acc: {}", acc);
                    break;
                }
            },
            _ => (),
        }
    }
}

fn run_program(program: &Vec<Op>) -> i32 {
    let max = program.len();
    let mut cpu = Cpu {
        pc: 0,
        acc: 0
    };
    for _ in 0..2*max {
        step(program, &mut cpu);
        if cpu.pc >= max as i32 {
            return cpu.acc;
        }
    }
    return 0;
}

fn step(program: &Vec<Op>, cpu: &mut Cpu) {
    match program[cpu.pc as usize] {
        Op::NOP(_) => (),
        Op::ACC(i) => cpu.acc+= i,
        Op::JMP(i) => cpu.pc += i-1,
    }
    cpu.pc += 1;
}

fn load() -> Vec<Op> {
    let input = tools::load(FNAME);
    let mut program = Vec::new();
    for line in input {
        let parts = line.split_whitespace().collect_vec();
        if let Some(i) = parts.get(0) {
            let op = match *i {
                "nop" => Op::NOP(parts.get(1).unwrap().parse().unwrap()),
                "acc" => Op::ACC(parts.get(1).unwrap().parse().unwrap()),
                "jmp" => Op::JMP(parts.get(1).unwrap().parse().unwrap()),
                _ => Op::NOP(0),
            };
            program.push(op);
        }
    }
    return program;
}