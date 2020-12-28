use std::intrinsics::transmute;
use std::collections::HashSet;

mod intcode;

const FNAME: &str = "./input/2019/2019-17.txt";

pub fn main() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program.clone_from(&program);
    intcode::reboot_computer(&mut comp);

    while intcode::step(&mut comp) {}
    println!("{:?}", comp);
    print_output(&comp);

    let map = output_to_map(&comp);
    println!("{:?}", map);
    print_map(&map);
    let intersections = get_intersections(&map);
    let mut s = 0;
    for q in &intersections {
        s += q.0 * q.1;
    }
    println!("{:?}", intersections);
    println!("Alignment Parameter Sum: {}", s);

    let path = trace_path_new(&map);
    let path = path.join("");
    println!("Path: {:?}", path);
    //path_to_code(&path);

    // Manually compress offline
    let main_routine = "A,B,A,C,B,A,B,C,C,B\n";
    let fA = "L,12,L,12,R,4\n";
    let fB = "R,10,R,6,R,4,R,4\n";
    let fC = "R,6,L,12,L,12\n";
    let input = string_to_ints(&vec![main_routine, fA, fB, fC, "n\n"].join(""));
    println!("input: {:?}", input);

    intcode::reboot_computer(&mut comp);
    comp.memory[0] = 2;
    comp.input.extend(&input);
    while intcode::step(&mut comp) {}
    println!("output: {:?}", comp.output);
    println!("last output: {:?}", comp.output.last());
}

fn string_to_ints(s: &str) -> Vec<i64> {
    s.chars().map(|c| c as i64).collect()
}

fn path_to_code(path: &String) {
    for n in 1..=path.len() {
        let sub = &path[0..n];
        let mut cnt = 0;
        for o in 0..(path.len() - n) {
            if sub == &path[o..(o + n)] {
                cnt += 1;
            }
        }
        let gain = 2 * (cnt - 1) * (sub.len()) as i32 - cnt * 2;
        println!("{:?}: {} gain: {}", sub, cnt, gain);
    }
}

fn trace_path_new(map: &Vec<Vec<i64>>) -> Vec<String> {
    let mut out = Vec::new();
    let nr = map[0].len();
    let nc = map.len();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = (0, 0);
    'outer: for y in 0..nr {
        for x in 0..nc {
            if match std::char::from_u32(map[x][y] as u32).unwrap() {
                '^' => {
                    dir.1 = -1;
                    true
                }
                'v' => {
                    dir.1 = 1;
                    true
                }
                '<' => {
                    dir.0 = -1;
                    true
                }
                '>' => {
                    dir.0 = 1;
                    ;
                    true
                }
                _ => false,
            } {
                pos.0 = x as i32;
                pos.1 = y as i32;
                break 'outer;
            }
        }
    }
    let mut cnt = 0;
    loop {
        // Check ahead
        let pos_n = (pos.0 + dir.0, pos.1 + dir.1);
        if map.get(pos_n.0 as usize).and_then(|m| m.get(pos_n.1 as usize)).unwrap_or(&0) == &35 { // 35 is #
            cnt += 1;
            pos.0 += dir.0;
            pos.1 += dir.1;
            continue;
        }
        // Check left
        let dir_l = match dir {
            (1, 0) => (0, -1), // right to up
            (-1, 0) => (0, 1), // left to down
            (0, 1) => (1, 0), // down to right
            (0, -1) => (-1, 0), // up to left
            _ => (0, 0),
        };
        if map.get((pos.0 + dir_l.0) as usize).and_then(|m| m.get((pos.1 + dir_l.1) as usize)).unwrap_or(&0) == &35 { // 35 is #
            if cnt > 0 {
                out.push(cnt.to_string());
                cnt = 0;
            }
            out.push("L".to_string());
            dir = dir_l;
            continue;
        }
        // Check right
        let mut dir_r = match dir {
            (1, 0) => (0, 1), // right to down
            (-1, 0) => (0, -1), // left to up
            (0, 1) => (-1, 0), // down to left
            (0, -1) => (1, 0), // up to right
            _ => (0, 0),
        };
        if map.get((pos.0 + dir_r.0) as usize).and_then(|m| m.get((pos.1 + dir_r.1) as usize)).unwrap_or(&0) == &35 { // 35 is #
            if cnt > 0 {
                out.push(cnt.to_string());
                cnt = 0;
            }
            out.push("R".to_string());
            dir = dir_r;
            continue;
        }
        if cnt > 0 {
            out.push(cnt.to_string());
            cnt = 0;
        }
        break;
    }
    return out;
}

fn trace_path(map: &Vec<Vec<i64>>) -> Vec<String> {
    // Find start
    let nr = map[0].len();
    let nc = map.len();
    let mut start = (0, 0);
    let mut dx = (0, 0);
    'outer: for ir in 0..nr {
        for ic in 0..nc {
            if match std::char::from_u32(map[ic][ir] as u32).unwrap() {
                '^' => {
                    dx = (0, -1);
                    true
                }
                'v' => {
                    dx = (0, 1);
                    true
                }
                '<' => {
                    dx = (-1, 0);
                    true
                }
                '>' => {
                    dx = (1, 0);
                    true
                }
                _ => false,
            } {
                start = (ic as i32, ir as i32);
                break 'outer;
            }
        }
    }
    let mut cnt = 0;
    let mut commands = Vec::new();

    loop {
        // Find next direction
        let mut end_dir = (0, 0);
        for d in 0..4 {
            let tx = match d {
                0 => (0, -1),
                1 => (0, 1),
                2 => (-1, 0),
                3 => (1, 0),
                _ => (0, 0),
            };
            if dx.0 == -tx.0 && dx.1 == -tx.1 {
                continue;
            }
            let c = map.get((start.0 + tx.0) as usize).and_then(|m| m.get((start.1 + tx.1) as usize)).unwrap_or(&0);
            if std::char::from_u32(*c as u32).unwrap() == '#' {
                end_dir = tx;
                break;
            }
        }
        // Map next turn to command
        if end_dir.0 == 0 && end_dir.1 == 0 {
            // Finished
            commands.push(cnt.to_string());
            cnt = 0;
            break;
        } else if (dx.0 == 0 && dx.1 != end_dir.1) || (dx.1 == 0 && dx.0 == end_dir.0) {
            // Right
            if cnt > 0 {
                commands.push(cnt.to_string());
                cnt = 0;
            }
            commands.push("R".to_string());
        } else {
            // Left
            if cnt > 0 {
                commands.push(cnt.to_string());
                cnt = 0;
            }
            commands.push("L".to_string());
        }
        // Move straight until can't
        loop {
            let x = start.0 + end_dir.0;
            let y = start.1 + end_dir.1;
            let c = map.get(x as usize).and_then(|m| m.get(y as usize)).unwrap_or(&0);
            if std::char::from_u32(*c as u32).unwrap() != '#' {
                break;
            } else {
                cnt += 1;
                start.0 += end_dir.0;
                start.1 += end_dir.1;
            }
        }
        dx = end_dir;
    }
    return commands;
}

fn get_intersections(map: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let nr = map[0].len();
    let nc = map.len();
    let mut out = Vec::new();
    for ir0 in 1..(nr - 1) {
        'outer: for ic0 in 1..(nc - 1) {
            for (ir, ic) in vec![(ir0, ic0), (ir0 + 1, ic0), (ir0 - 1, ic0), (ir0, ic0 + 1), (ir0, ic0 - 1)] {
                match std::char::from_u32(map[ic][ir] as u32).unwrap() {
                    '#' | '^' | 'v' | '<' | '>' => continue,
                    _ => continue 'outer,
                };
            }
            out.push((ic0, ir0));
        }
    }
    return out;
}

fn print_map(map: &Vec<Vec<i64>>) {
    println!("nr: {} nc:{} ", map[0].len(), map.len());
    for ir in 0..map[0].len() {
        for ic in 0..map.len() {
            print!("{}", std::char::from_u32(map[ic][ir] as u32).unwrap());
        }
    }
}

fn output_to_map(comp: &intcode::Computer) -> Vec<Vec<i64>> {
    let mut nc = 0;
    for i in 0..comp.output.len() {
        if &comp.output[i] == &10 {
            nc = i + 1;
            break;
        }
    }
    let nr = comp.output.len() / nc;
    let mut map = vec![vec![0; nr]; nc];
    for ir in 0..nr {
        for ic in 0..nc {
            map[ic][ir] = comp.output[ir * nc + ic];
        }
    }
    return map;
}

fn print_output(comp: &intcode::Computer) {
    for i in &comp.output {
        print!("{}", std::char::from_u32(*i as u32).unwrap());
        /*match i {
            &35 => print!("{}", std::char::from_u32(*i as u32).unwrap()),
            &10 => print!("\n"),
            _ => print!(" "),
        }

         */
    }
    println!();
}