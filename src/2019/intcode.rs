#[derive(Debug)]
pub struct Computer {
    pub program: Vec<i64>,
    pub memory: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub pc: usize,
    pub relative_base: usize,
}

/*pub fn main() {
    println!("Intcode");
    let mut comp = Computer {
        program: Vec::new(),
        memory: vec![1101,100,-1,4,0],
        input: Vec::new(),
        output: Vec::new(),
        pc: 0,
    };
    while step(&mut comp) {
        println!("{:?}", comp);
    }
}*/

pub fn step(comp: &mut Computer) -> bool {
    let (op, params) = fetch_params(&comp);
    operate(comp, op, &params)
}

fn get_n(op: i64) -> usize {
    match op {
        1..=2 => 4,
        3..=4 => 2,
        5..=6 => 3,
        7..=8 => 4,
        9 => 2,
        99 => 1,
        _ => 0,
    }
}

fn fetch_params(comp: &Computer) -> (i64, Vec<usize>) {
    let op_raw = comp.memory[comp.pc];
    let op_int = op_raw%100;
    let mode_int = (op_raw - op_int) / 100;
    let n = get_n(op_int);
    let mut params = vec![0; n - 1];
    for i in 0..n-1 {
        let mode = (mode_int / (10 as i64).pow(i as u32)) % 10;
        //println!("ind: {}, Mode: {}, mode_int: {}, op_int: {}", comp.pc + 1 + i, mode, mode_int, op_int);
        // Return parameter reference location
        match mode {
            0 => params[i] = comp.memory[comp.pc + 1 + i] as usize,
            1 => params[i] = (comp.pc + 1 + i) as usize,
            2 => params[i] = (comp.relative_base as i64 + comp.memory[comp.pc + 1 + i]) as usize,
            _ => (),
        }
    }
    return (op_int, params);
}

pub fn operate(comp: &mut Computer, op: i64, params: &Vec<usize>) -> bool {
    //dbg!(op, params);

    for p in params {
        if p >= &comp.memory.len() {
            let n = p + 1 - &comp.memory.len();
            &comp.memory.extend(vec![0; n]);
        }
    }

    let mut dpc = params.len() + 1;
    match op {
        1 => comp.memory[params[2]] = comp.memory[params[0]] + comp.memory[params[1]],
        2 => comp.memory[params[2]] = comp.memory[params[0]] * comp.memory[params[1]],
        3 => {
            if comp.input.is_empty() {
                //println!("empty");
                return false;
            } else {
                comp.memory[params[0]] = comp.input.remove(0);
            }
        },
        4 => comp.output.push(comp.memory[params[0]]),
        5 => if comp.memory[params[0]] != 0 {
            comp.pc = comp.memory[params[1]] as usize;
            dpc = 0;
        },
        6 => if comp.memory[params[0]] == 0 {
            comp.pc = comp.memory[params[1]] as usize;
            dpc = 0;
        },
        7 => comp.memory[params[2]] = if comp.memory[params[0]] < comp.memory[params[1]] { 1 } else { 0 },
        8 => comp.memory[params[2]] = if comp.memory[params[0]] == comp.memory[params[1]] { 1 } else { 0 },
        9 => comp.relative_base = (comp.relative_base as i64 + comp.memory[params[0]]) as usize,
        99 => {
            //println!("done");
            return false;
        },
        _ => (),
    }
    comp.pc += dpc;
    return true;
}

pub fn load_program(fname: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(fname).unwrap();
    let a = input.split(",").flat_map(|x| x.trim().parse()).collect();
    //println!{"{:?}", a};
    return a;
}

pub fn new_computer() -> Computer {
    Computer {
        program: Vec::new(),
        memory: Vec::new(),
        input: Vec::new(),
        output: Vec::new(),
        pc: 0,
        relative_base: 0,
    }
}

pub fn reboot_computer(comp: &mut Computer) {
    comp.memory = comp.program.clone();
    comp.input.clear();
    comp.output.clear();
    comp.pc = 0;
    comp.relative_base = 0;
}

pub fn setup_new_computer(program_file: &str) -> Computer {
    let mut comp = new_computer();
    comp.program = load_program(program_file);
    reboot_computer(&mut comp);
    return comp;
}
