mod intcode;
const FNAME: &str = "./input/2019/2019-25.txt";
pub fn main() {
    let mut comp = intcode::setup_new_computer(FNAME);
    let mut query = String::new();
    let mut save_state = intcode::setup_new_computer(FNAME);
    let mut save_room_msg = Vec::new();
    loop {
        // println!("{:?}", comp.input.iter().map(|x| *x as u8 as char).collect::<Vec<char>>());
        while intcode::step(&mut comp) {}
        for c in &comp.output {
            print!("{}", *c as u8 as char);
        }
        comp.input.clear();
        query.clear();
        std::io::stdin().read_line(&mut query).ok().expect("Bad input");
        if query.starts_with("exit") {
            println!("Goodbye");
            break;
        } else if query.starts_with("save") {
            save_room_msg = comp.output.clone();
            save_state.pc = comp.pc.clone();
            save_state.memory = comp.memory.clone();
            save_state.output = comp.output.clone();
            save_state.relative_base = comp.relative_base.clone();
            println!("State Saved");
        } else if query.starts_with("load") {
            comp.memory = save_state.memory.clone();
            comp.pc = save_state.pc.clone();
            comp.output = save_state.output.clone();
            comp.relative_base = save_state.relative_base.clone();
        } else {
            comp.output.clear();
            for c in query.chars() {
                comp.input.push(c as i64);
            }
        }
    }

}
