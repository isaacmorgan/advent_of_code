use std::collections::HashMap;

mod intcode;

const FNAME: &str = "./input/2019/2019-11.txt";

pub fn main() {
    let mut robot = [0; 3];
    let mut map = HashMap::new();
    let mut comp = intcode::new_computer();
    comp.program = intcode::load_program(FNAME);
    intcode::reboot_computer(&mut comp);

    // Part 1
    //comp.input.push(0);
    // Part 2
    comp.input.push(1);

    let mut cnt = 0;
    while intcode::step(&mut comp) {
        //println!("{:?}", &comp);
        if &comp.output.len() == &2 {
            // Read color and paint
            let color = comp.output.remove(0);
            map.insert([robot[0], robot[1]], color);

            // Rotate
            robot[2] += match comp.output.remove(0) {
                0 => -1,
                1 => 1,
                _ => 0,
            };
            robot[2] = (robot[2] as i32).rem_euclid(4);

            // Move
            match robot[2] {
                0 => robot[1] -= 1,
                1 => robot[0] += 1,
                2 => robot[1] += 1,
                3 => robot[0] -= 1,
                _ => println!("Wtf: {}", robot[2]),
            }
            //println!("Map: {:?} Robot: {:?}", &map, &robot);

            // Set color as input
            comp.input.push(*match map.contains_key(&[robot[0], robot[1]]) {
                true => map.get(&[robot[0], robot[1]]).unwrap(),
                false => &0,
            });
        }
    }

    println!("Num. painted panels: {}", map.len());

    print_map(&map);
}

fn print_map(map: &HashMap<[i32; 2], i64>) {
    if map.is_empty() {
        return
    }
    let xmin = map.iter().map(|(a,_)| a[0]).min().unwrap();
    let xmax = map.iter().map(|(a,_)| a[0]).max().unwrap();
    let ymin = map.iter().map(|(a,_)| a[1]).min().unwrap();
    let ymax = map.iter().map(|(a,_)| a[1]).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&[x, y]) {
                match map.get(&[x, y]).unwrap() {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ => (),
                }
            } else {
                print!(" ")
            }
        }
        println!();
    }
    println!("Min: {},{} Max: {},{}", xmin, ymin, xmax, ymax);
}