const FNAME: &str = "./input/2019/2019-02.txt";
const S: usize = 4;

pub fn main() {
    let mut program = load();
    program[1] = 12;
    program[2] = 2;
    println!("{:?}", program);
    let mut pc = 0;
    while step(pc, &mut program) {
        pc += S;
        //println!("{:?}", program);
    }
    println!("pos [0]: {}", program[0]);

    'outer:
    for n in 0..100 {
        for v in 0..100 {
            program = load();
            program[1] = n;
            program[2] = v;
            pc = 0;
            while step(pc, &mut program) {
                pc += S;
                //println!("{:?}", program);
            }
            if program[0] == 19_690_720 {
                println!("noun+verb: {} pos [0]: {}", n*100 + v, program[0]);
                break 'outer;
            }
        }
    }
}

fn step(pc: usize, program: &mut Vec<usize>) -> bool {
    match program[pc] {
        1 => {
            //println!("Opcode 1");
            let a = program[pc+1];
            let b = program[pc+2];
            let c = program[pc+3];
            program[c] = program[a] + program[b];
            true
        },
        2 => {
            //println!("Opcode 2");
            let a = program[pc+1];
            let b = program[pc+2];
            let c = program[pc+3];
            program[c] = program[a] * program[b];
            true
        },
        99 => false,
        _ => {
            println!("Unknown Opcode\n\tpc: {} code: {}", pc, program[pc]);
            false
        },
    }
}

fn load() -> Vec<usize> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    input.split(",").flat_map(|x| x.parse()).collect()
}