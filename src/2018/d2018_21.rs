use regex::Regex;
use std::fs;
use std::borrow::BorrowMut;
use std::collections::HashMap;

const NOOP: i64 = -1;
const ADDR: i64 = 0;
const ADDI: i64 = 1;
const MULR: i64 = 2;
const MULI: i64 = 3;
const BANR: i64 = 4;
const BANI: i64 = 5;
const BORR: i64 = 6;
const BORI: i64 = 7;
const SETR: i64 = 8;
const SETI: i64 = 9;
const GTIR: i64 = 10;
const GTRI: i64 = 11;
const GTRR: i64 = 12;
const EQIR: i64 = 13;
const EQRI: i64 = 14;
const EQRR: i64 = 15;

const NR: usize = 6;
const FNAME: &str = "./input/2018/2018-21.txt";

pub fn main() {
    let (mut cpu, program) = load();
    print_cpu(&cpu);
    cpu.reg[0] = 7700742;
    let mut cnt = 0;
    let mut stack = Vec::new();
    while step(&program, &mut cpu) {
        cnt += 1;
        //print_cpu(&cpu);
        if cpu.ip == 28 {
            print_cpu(&cpu);
            if stack.contains(&cpu.reg[4]) {
                break;
            }
            stack.push(cpu.reg[4])
        }
    }
    print_cpu(&cpu);
    println!("Steps: {}", cnt);
}

fn print_cpu(cpu: &Cpu) {
    println!("ip: {} reg: {:?}", cpu.ip, cpu.reg);
}

fn load() -> (Cpu, Program) {
    let mut contents = fs::read_to_string(FNAME).expect("Error reading file");

    let mut opcode = Vec::new();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();

    let mut cpu = Cpu {
        reg: [0; NR],
        ip: 0,
        ipreg: 0,
    };

    cpu.reg[0] = 1;

    for line in contents.split("\n") {
        // IP command
        if line.starts_with('#') {
            let re = Regex::new(r"^#ip (\d+)").unwrap();
            for cap in re.captures_iter(line) {
                cpu.ipreg = cap[1].parse::<usize>().unwrap();
            }
        } else if !line.is_empty() {
            // Regular opcodes
            let re = Regex::new(r"^(.{4}) (\d+) (\d+) (\d+)").unwrap();
            for cap in re.captures_iter(line) {
                opcode.push(op_to_int(&cap[1]));
                a.push(cap[2].parse::<usize>().unwrap());
                b.push(cap[3].parse::<usize>().unwrap());
                c.push(cap[4].parse::<usize>().unwrap());
            }
        }
    }

    let n = opcode.len();
    let program = Program{
        opcode,
        a,
        b,
        c,
        n,
    };

    return (cpu, program);
}

fn step(program: &Program, cpu: &mut Cpu) -> bool {
    if cpu.ip >= program.n {
        return false;
    }

    if true && cpu.ip == 17 {
        if cpu.reg[1]%256 == 0 {
            cpu.reg[1] = cpu.reg[1]/256;
        } else {
            cpu.reg[1] = cpu.reg[1]/256;
        }
        cpu.ip = 8;
        return true
    }

    operate(program, cpu);

    cpu.ip += 1;

    return true;
}

fn op_to_int(opcode: &str) -> i64 {
    match opcode {
        "addr" => ADDR,
        "addi" => ADDI,
        "mulr" => MULR,
        "muli" => MULI,
        "banr" => BANR,
        "bani" => BANI,
        "borr" => BORR,
        "bori" => BORI,
        "setr" => SETR,
        "seti" => SETI,
        "gtir" => GTIR,
        "gtri" => GTRI,
        "gtrr" => GTRR,
        "eqir" => EQIR,
        "eqri" => EQRI,
        "eqrr" => EQRR,
        _ => NOOP,
    }
}

fn int_to_op(opcode: i64) -> String {
    match opcode {
        ADDR => "addr",
        ADDI => "addi",
        MULR => "mulr",
        MULI => "muli",
        BANR => "banr",
        BANI => "bani",
        BORR => "borr",
        BORI => "bori",
        SETR => "setr",
        SETI => "seti",
        GTRI => "gtri",
        GTIR => "gtir",
        GTRR => "gtrr",
        EQIR => "eqir",
        EQRI => "eqri",
        EQRR => "eqrr",
        _ => "noop",
    }.to_string()
}

fn operate(program: &Program, cpu: &mut Cpu) {
    let mut reg = &mut cpu.reg;

    let inst = program.opcode[cpu.ip];
    let a = program.a[cpu.ip];
    let b = program.b[cpu.ip];
    let c = program.c[cpu.ip];

    reg[cpu.ipreg] = cpu.ip as i64;

    //println!("ip: {} reg: {:?} inst: {} {} {} {}", cpu.ip, reg, int_to_op(inst), a, b, c);

    match inst {
        ADDR => {
            reg[c] = reg[a] + reg[b];
        },
        ADDI => {
            reg[c] = reg[a] + b as i64;
        },
        MULR => {
            reg[c] = reg[a]*reg[b];
        },
        MULI => {
            reg[c] = reg[a]*(b as i64);
        },
        BANR => {
            reg[c] = reg[a] & reg[b];
        },
        BANI => {
            reg[c] = reg[a] & (b as i64);
        },
        BORR => {
            reg[c] = reg[a] | reg[b];
        },
        BORI => {
            reg[c] = reg[a] | (b as i64);
        },
        SETR => {
            reg[c] = reg[a];
        },
        SETI => {
            reg[c] = a as i64;
        },
        GTIR => {
            reg[c] = if a as i64 > reg[b] { 1 } else { 0 }
        },
        GTRI => {
            reg[c] = if reg[a] > b as i64 { 1 } else { 0 }
        },
        GTRR => {
            reg[c] = if reg[a] > reg[b] { 1 } else { 0 }
        },
        EQIR => {
            reg[c] = if a as i64 == reg[b] { 1 } else { 0 }
        },
        EQRI => {
            reg[c] = if reg[a] == b as i64 { 1 } else { 0 }
        },
        EQRR => {
            reg[c] = if reg[a] == reg[b] { 1 } else { 0 }
        },
        _ => (),
    }

    cpu.ip = cpu.reg[cpu.ipreg] as usize;
}

struct Cpu {
    reg: [i64; NR],
    ip: usize,
    ipreg: usize,
}

struct Program {
    opcode: Vec<i64>,
    a: Vec<usize>,
    b: Vec<usize>,
    c: Vec<usize>,
    n: usize,
}
