use std::collections::HashMap;

mod intcode;

const FNAME: &str = "./input/2019/2019-13.txt";

pub fn main() {
    let mut comp = intcode::new_computer();
    comp.program = intcode::load_program(FNAME);
    intcode::reboot_computer(&mut comp);
    while intcode::step(&mut comp) {}
    let mut block_cnt = 0;
    for i in (2..comp.output.len()).step_by(3) {
        if comp.output.get(i).unwrap() == &2 {
            block_cnt += 1;
        }
    }
    println!("Num blocks: {}", block_cnt);

    intcode::reboot_computer(&mut comp);
    comp.memory[0] = 2;

    let mut screen = HashMap::new();
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut cnt = 0;
    while intcode::step(&mut comp) {
        cnt += 1;
        if comp.output.len() >= 3 {
            while comp.output.len() >= 3 {
                let x = comp.output.remove(0);
                let y = comp.output.remove(0);
                let z = comp.output.remove(0);
                if x == -1 && y == 0 {
                    score = z;
                } else {
                    screen.insert([x, y], z);
                    match z {
                        3 => paddle_x = x,
                        4 => ball_x = x,
                        _ => (),
                    }
                }
            }
            //print_screen(&screen);
            //println!("input: {:?}", comp.input);
            comp.input.clear();
            if paddle_x < ball_x {
                comp.input.push(1);
            } else if paddle_x > ball_x {
                comp.input.push(-1);
            } else {
                comp.input.push(0);
            }
        }
        if cnt % 1000 == 0 {
            print_screen(&screen);
            println!("Score: {}", score);
        }
    }
    println!("Final score: {}", score);
}

fn print_screen(screen: &HashMap<[i64; 2], i64>) {
    if screen.is_empty() {
        return
    }

    let xmin = screen.keys().map(|&x| x[0]).min().unwrap();
    let ymin = screen.keys().map(|&x| x[1]).min().unwrap();
    let xmax = screen.keys().map(|&x| x[0]).max().unwrap();
    let ymax = screen.keys().map(|&x| x[1]).max().unwrap();
    println!("xlim: {}..={} ylim: {}..={}", xmin, xmax, ymin, ymax);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            match screen.get(&[x,y]) {
                Some(x) => {
                    let c = match x {
                        0 => " ",
                        1 => "|",
                        2 => "#",
                        3 => "=",
                        4 => "o",
                        _ => "?",
                    };
                    print!("{}", c);
                },
                None => print!(" "),
            }
        }
        println!();
    }
}
