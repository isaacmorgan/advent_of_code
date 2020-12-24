use std::collections::HashMap;
use rand::random;
use std::cmp::{min, max};

mod intcode;

const FNAME: &str = "./input/2019/2019-15.txt";

pub fn main() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program = program;
    intcode::reboot_computer(&mut comp);

    // Run program to explore map
    // Check for any empty spots
    // Run A* to find path to empty spot
    // Repeat until no empty spots
    // Run A* to find path from start to finish
    // Run flood fill to count how many minutes to fill map

    let mut pos = (0, 0);
    let mut map = HashMap::new();
    for i in 0..1000000 {
        let input = random::<i64>().rem_euclid(4) + 1;
        let dpos = match input {
            1 => (0, 1),
            2 => (0, -1),
            3 => (-1, 0),
            4 => (1, 0),
            _ => (0, 0),
        };
        comp.input.push(input);
        intcode::step(&mut comp);
        loop {
            if comp.output.len() == 0 {
                intcode::step(&mut comp);
            } else {
                break;
            }
        }
        let status = comp.output.remove(0);
        match status {
            0 => {
                let wall_p = (pos.0 + dpos.0, pos.1 + dpos.1);
                map.insert(wall_p, '#');
            },
            1 => {
                pos.0 += dpos.0;
                pos.1 += dpos.1;
                map.insert(pos.clone(), '.');
            },
            2 => {
                pos.0 += dpos.0;
                pos.1 += dpos.1;
                map.insert(pos.clone(), 'O');
            },
            _ => (),
        }
    }
    map.insert(pos.clone(), 'D');
    map.insert((0,0), 'S');
    print_map(&map);
}

fn print_map(map: &HashMap<(i32, i32), char>) {
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for (p,_) in map {
        xmin = min(xmin, p.0);
        xmax = max(xmax, p.0);
        ymin = min(ymin, p.1);
        ymax = max(ymax, p.1);
    }

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            print!("{}", map.get(&(x,y)).unwrap_or(&' '));
        }
        println!();
    }
}
