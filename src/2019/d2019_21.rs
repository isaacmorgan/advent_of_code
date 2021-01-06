mod intcode;

const FNAME: &str = "./input/2019/2019-21.txt";

pub fn main() {
    let mut comp = intcode::setup_new_computer(FNAME);

    run_springcode(&mut comp, "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nNOT D T\nNOT T T\nAND T J\nWALK\n");
    run_springcode(&mut comp, "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nNOT D T\nNOT T T\nAND T J\nNOT E T\nNOT T T\nOR H T\nAND T J\nRUN\n");
}

fn run_springcode(comp: &mut intcode::Computer, program: &str) {
    intcode::reboot_computer(comp);
    
    for i in program.chars() {
        comp.input.push(i as i64);
    }

    while intcode::step(comp) {}

    // println!("{:?}", &comp.output);

    for i in &comp.output {
        if i > &255 {
            print!("Hull Damage: {}", i);
            continue;
        }
        print!("{}", (*i as u8) as char);
    }
    println!();
    println!();
}