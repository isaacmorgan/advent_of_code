mod intcode;

const FNAME: &str = "./input/2019/2019-09.txt";

pub fn main() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program = program.clone();
    comp.memory = comp.program.clone();

    comp.input.push(1);

    while intcode::step(&mut comp) {}
    println!("Output: {:?}", comp.output);

    intcode::reboot_computer(&mut comp);

    comp.input.push(2);

    while intcode::step(&mut comp) {}
    println!("Output: {:?}", comp.output);
}
