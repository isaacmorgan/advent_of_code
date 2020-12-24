mod intcode;

const FNAME: &str = "./input/2019/2019-05.txt";

pub fn main() {
    let mut comp = intcode::Computer {
        program: Vec::new(),
        memory: Vec::new(),
        input: Vec::new(),
        output: Vec::new(),
        pc: 0,
    };
    print_mem(&comp.memory);
    comp.memory = intcode::load_program(FNAME);
    comp.input.push(5);

    while intcode::step(&mut comp) {
        println!("{:?}", comp);
        print_mem(&comp.memory);
    }

    println!("Diagnostic code: {:?}", &comp.output);
}

fn print_mem(mem: &Vec<i32>) {
    let mut cnt = 0;
    for m in mem {
        if cnt%10 == 0 {
            print!("\n{}:", cnt);
        }
        print!(", {}", m);
        cnt += 1;
    }
    println!();
}