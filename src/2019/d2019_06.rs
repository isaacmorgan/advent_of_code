use std::collections::HashMap;

const FNAME: &str = "./input/2019/2019-06.txt";

pub fn main() {
    let c2p = load();
    let mut cnt = 0;
    for c in c2p.keys() {
        cnt += count_orbits(&c2p, c);
    }
    println!("{:?}", c2p);
    println!("Total number of orbits: {:?}", cnt);

    let min_transfer = count_min_transfer(&c2p, &"YOU".to_string(), &"SAN".to_string());
    println!("Min number of orbital transfers: {}", min_transfer);
}

fn count_min_transfer(c2p: &HashMap<String, String>, a: &String, b: &String) -> i32 {
    let a_p = get_parents(&c2p, a);
    let b_p = get_parents(&c2p, b);

    let mut shared_p = "".to_string();
    for p in a_p {
        if b_p.contains(&p) {
            shared_p = p;
            break;
        }
    };
    println!("Shared parent: {}", shared_p);

    return count_to_parent(&c2p, a, &shared_p) + count_to_parent(&c2p, b, &shared_p) - 2;
}

fn count_to_parent(c2p: &HashMap<String, String>, start: &String, end: &String) -> i32 {
    let mut cnt = 0;
    let mut c = start;
    while c2p.contains_key(c) {
        cnt += 1;
        c = c2p.get(c).unwrap();
        if c == end {
            break;
        }
    }
    if c != end {
        0
    }  else {
        cnt
    }
}

fn get_parents(c2p: &HashMap<String, String>, name: &String) -> Vec<String> {
    let mut parents = Vec::new();
    let mut c = name;
    while c2p.contains_key(c) {
        c = c2p.get(c).unwrap();
        parents.push(c.clone());
    }
    return parents;
}

fn count_orbits(c2p: &HashMap<String, String>, name: &String) -> i32 {
    let mut cnt = 0;
    let mut c = name;
    while c2p.contains_key(c) {
        cnt += 1;
        c = c2p.get(c).unwrap();
    }
    return cnt;
}

fn load() -> HashMap<String, String> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    let re = regex::Regex::new(r"(?m:^([^\)\s]+)\)([^\)\s]+)\s*$)").unwrap();

    let mut map_c2p = HashMap::new();
    for cap in re.captures_iter(&input) {
        map_c2p.insert(cap[2].to_string(), cap[1].to_string());
    }
    return map_c2p;
}
