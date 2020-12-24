use std::collections::HashSet;
use std::iter::FromIterator;

const FNAME: &str = "./input/2018/2018-25.txt";
const D: i32 = 3;

pub fn main() {
    let points = load();
    println!("{:?}", points);
    let groups = find_groups(&points);
    println!("{:?}", groups);
    let set: HashSet<&i32> = HashSet::from_iter(groups.iter());
    println!("There are {} constellations!", set.len());
}

fn find_groups(points: &Vec<[i32; 4]>) -> Vec<i32> {
    let mut groups = vec![0; points.len()];
    let mut cnt = 1;
    for i in 0..points.len() {
        if groups[i] == 0 {
            groups[i] = cnt;
            cnt += 1;
        }
        for j in i..points.len() {
            //println!("a: {:?} b: {:?} nearby: {}", points[i], points[j], is_nearby(&points[i], &points[j]));
            if is_nearby(&points[i], &points[j]) {
                if groups[j] == 0 {
                    groups[j] = groups[i];
                }
                if groups[j] > groups[i] {
                    // set all js to is
                    let jval = groups[j];
                    for k in 0..points.len() {
                        if groups[k] == jval {
                            groups[k] = groups[i]
                        }
                    }
                } if groups[i] > groups[j] {
                    // set all is to js
                    let ival = groups[i];
                    for k in 0..points.len() {
                        if groups[k] == ival {
                            groups[k] = groups[j];
                        }
                    }
                }
            }

            //println!("{:?}", groups);
        }
    }
    return groups;
}

fn is_nearby(a: &[i32; 4], b: &[i32; 4]) -> bool {
    let mut s = 0;
    for i in 0..4 {
        s += (a[i] - b[i]).abs();
    }
    s <= D
}

fn load() -> Vec<[i32; 4]> {
    let contents = std::fs::read_to_string(FNAME).expect("Error reading file");
    let mut points = Vec::new();
    for c in contents.split("\n") {
        if c.is_empty() {
            continue;
        }
        let mut cnt = 0;
        let mut point = [0; 4];
        for d in c.split(",") {
            point[cnt] = d.trim().parse::<i32>().unwrap();
            cnt += 1;
        }
        points.push(point);
    }
    return points;
}