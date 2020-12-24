use std::i32::MAX;
use std::cmp::{min, max};

const FNAME: &str = "./input/2019/2019-03.txt";

#[derive(Debug)]
struct Wire {
    x: [i32; 2],
    y: [i32; 2],
}

pub fn main() {
    let (a, b) = load();
    println!("{:?}", a);
    println!("{:?}", b);

    let wires_a = dir_to_wires(&a);
    let wires_b = dir_to_wires(&b);

    println!("{:?}", wires_a);
    println!("{:?}", wires_b);

    let mut min_dist = MAX;
    for wa in &wires_a {
        for wb in &wires_b {
            match check_wires_cross(&wa, &wb) {
                None => (),
                Some(x) => {
                    let val = x[0].abs() + x[1].abs();
                    //println!("dist: {}, a: {:?}, b: {:?}, int: {:?}", val, wa, wb, x);
                    if val < min_dist && val > 0 {
                        println!("dist: {}, a: {:?}, b: {:?}, int: {:?}", val, wa, wb, x);
                        min_dist = val;
                    }
                }
            }
        }
    }

    println!("Dist. to nearest intersection: {}", min_dist);

    let crosses = get_crosses(&wires_a, &wires_b);

    println!("Crosses: {:?}", crosses);

    let d = get_combined_dist(&wires_a, &wires_b, &crosses);
    println!("Best steps: {:?}", d.iter().min());
}

fn get_combined_dist(a: &Vec<Wire>, b: &Vec<Wire>, crosses: &Vec<Wire>) -> Vec<i32> {
    let trace_a = trace_wires(&a, &crosses);
    let trace_b = trace_wires(&b, &crosses);
    println!("Dist: {:?}", trace_a);
    println!("Dist: {:?}", trace_b);
    let mut total = Vec::new();
    for i in 0..trace_a.len() {
        total.push(trace_a[i] + trace_b[i]);
    }
    return total;
}

fn trace_wires(a: &Vec<Wire>, crosses: &Vec<Wire>) -> Vec<i32> {
    let len: Vec<i32> = a.iter().map(|w| (w.x[1] - w.x[0]).abs() + (w.y[1] - w.y[0]).abs()).collect();
    println!("Len: {:?}", len);
    //let mut len = Vec::new();
    //for w in a {
//        len.push((w.x[1] - w.x[0]).abs() + (w.y[1] - w.y[0]).abs());
//    }
    let mut out = Vec::new();
    for c in crosses {
        let mut d = 0;
        for i in 0..a.len() {
            if check_wires_cross(&a[i], c).is_some() {
                let dd = (c.x[0] - &a[i].x[0]).abs() + (c.y[0] - &a[i].y[0]).abs();
                d += (c.x[0] - &a[i].x[0]).abs() + (c.y[0] - &a[i].y[0]).abs();
                //println!("a: {:?}, c: {:?}, delta: {}, d: {}", &a[i], c, dd, d);
                out.push(d);
                break;
            } else {
                d += len[i];
                //println!("d: {}, delta: {}", d, len[i]);
            }
        }
    }
    return out;
}

fn get_crosses(a: &Vec<Wire>, b: &Vec<Wire>) -> Vec<Wire> {
    let mut crosses = Vec::new();
    for wa in a {
        for wb in b {
            match check_wires_cross(&wa, &wb) {
                None => (),
                Some(x) => {
                    let val = x[0].abs() + x[1].abs();
                    //println!("dist: {}, a: {:?}, b: {:?}, int: {:?}", val, wa, wb, x);
                    if val > 0 {
                        //println!("dist: {}, a: {:?}, b: {:?}, int: {:?}", val, wa, wb, x);
                        crosses.push(Wire{
                            x: [x[0], x[0]],
                            y: [x[1], x[1]],
                        });
                    }
                }
            }
        }
    }
    return crosses;
}

fn check_wires_cross(a: &Wire, b: &Wire) -> Option<[i32; 2]> {
    let axmin = min(a.x[0], a.x[1]);
    let axmax = max(a.x[0], a.x[1]);
    let aymin = min(a.y[0], a.y[1]);
    let aymax = max(a.y[0], a.y[1]);
    let bxmin = min(b.x[0], b.x[1]);
    let bxmax = max(b.x[0], b.x[1]);
    let bymin = min(b.y[0], b.y[1]);
    let bymax = max(b.y[0], b.y[1]);
    if a.y[0] != a.y[1] {
        // a is vertical
        if b.y[0] != b.y[1] {
            // b is vertical
            None
        } else {
            // b is horizontal
            if bxmin <= axmin && bxmax >= axmin
                && aymin <= bymin && aymax >= bymin {
                Some([axmin, bymin])
            } else {
                None
            }
        }
    } else {
        // a is horizontal
        if b.x[0] == b.x[1] {
            // b is vertical
            if axmin <= bxmin && axmax >= bxmin
                && bymin <= aymin && bymax >= aymin {
                Some([bxmin, aymin])
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn dir_to_wires(dir: &Vec<String>) -> Vec<Wire> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 0;
    let mut wires = Vec::new();
    for d in dir {
        let direction = &d[0..1].to_string();
        let n = d[1..d.len()].to_string().parse::<i32>().unwrap();
        match direction.as_str() {
            "U" => {
                dx = 0;
                dy = n;
            },
            "D" => {
                dx = 0;
                dy = -n;
            },
            "L" => {
                dx = -n;
                dy = 0;
            },
            "R" => {
                    dx = n;
                    dy = 0;
                },
            _ => (),
        }
        wires.push(Wire {
            x: [x, x+dx],
            y: [y, y+dy],
        });
        x += dx;
        y += dy;
    }
    return wires;
}

fn load() -> (Vec<String>, Vec<String>) {
    let input = std::fs::read_to_string(FNAME).unwrap();
    let mut tmp = input.split("\n").into_iter();
    let a = tmp.next().unwrap().split(",").map(|x| x.trim().to_string()).collect();
    let b = tmp.next().unwrap().split(",").map(|x| x.trim().to_string()).collect();
    return (a, b);
}