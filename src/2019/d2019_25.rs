mod intcode;
const FNAME: &str = "./input/2019/2019-25.txt";
pub fn main() {
    let mut comp = intcode::setup_new_computer(FNAME);
    loop {
        while (intcode::step(&mut comp)) {}
        for c in &comp.output {
            print!("{}", c as u8 as char);
        }
        comp.output.reset();
    }
}
