use crate::d2019_07::intcode::{step, Computer};
use itertools::Itertools;
mod intcode;

const FNAME: &str = "./input/2019/2019-07.txt";
const N: usize = 5;

pub fn main() {
    let program = intcode::load_program(FNAME);
    let mut amp_chain = create_amp_chain(N);

    // Part 1
    if false {
        let items = vec![0, 1, 2, 3, 4];
        let mut max_thrust = 0;
        let mut max_setting = Vec::new();
        for perm in items.iter().permutations(items.len()).unique() {
            init_amp_chain(&mut amp_chain, &perm);
            run_amp_chain(&mut amp_chain);
            let output = amp_chain.last().unwrap().output[0];
            println!("input: {:?} output: {:?}", &perm, output);
            if output > max_thrust {
                max_thrust = output;
                max_setting = perm.clone();
            }
        }

        println!("Max thrust {} using {:?}", max_thrust, max_setting);
    }

    // Part 2
    let items = vec![5, 6, 7, 8, 9];
    let mut max_thrust = 0;
    let mut max_setting = Vec::new();
    for perm in items.iter().permutations(items.len()).unique() {
        init_amp_chain(&mut amp_chain, &perm);
        let output = run_amp_chain_loop(&mut amp_chain);
        println!("input: {:?} output: {:?}", &perm, output);
        if output > max_thrust {
            max_thrust = output;
            max_setting = perm.clone();
        }
    }
    println!("Max thrust {} using {:?}", max_thrust, max_setting);
}

fn run_amp_chain_loop(amps: &mut Vec<intcode::Computer>) -> i32 {
    let mut output = 0;
    let mut i = 0;
    'outer: loop {
        let mut amp = &mut amps[i];
        amp.input.push(output);
        while step(&mut amp) {
            if !amp.output.is_empty() {
                //println!("{:?}", amp.output);
                output = amp.output.remove(0);
                i = (i+1) % N;
                continue 'outer;
            }
        }
        break;
    }
    return output;
}

fn run_amp_chain(amps: &mut Vec<intcode::Computer>) {
    let mut output = 0;
    for amp in amps {
        amp.input.push(output);
        while step(amp) {}
        output = amp.output[0];
    }
}

fn init_amp_chain(amps: &mut Vec<intcode::Computer>, inputs: &Vec<&i32>) {
    for i in 0..N {
        amps[i].memory = amps[i].program.clone();
        amps[i].input.clear();
        amps[i].output.clear();
        amps[i].pc = 0;
        amps[i].input.push(*inputs[i]);
    }
}

fn create_amp_chain(n: usize) -> Vec<intcode::Computer> {
    let program = intcode::load_program(FNAME);
    let mut amp_chain = Vec::new();
    for i in 0..n {
        let mut amp = intcode::new_computer();
        amp.program = program.clone();
        amp.memory = amp.program.clone();
        amp_chain.push(amp);
    }
    return amp_chain;
}

