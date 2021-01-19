use crate::tools;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-07.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (map, _) = load();
    // println!("map: {:?}", map);
    let mut parents = HashSet::new();
    let child = "shiny gold bag";
    get_all_parents(child, &mut parents, &map);
    println!("parents: {:?}", parents);
    println!("Number of bags that can contain {}: {}", child, parents.len());
}

fn part2() {
    let (_, map) = load();
    // println!("map: {:?}", map);
    let parent = "shiny gold bag";
    let cnt = count_all_children(parent, &map);
    println!("Number of bags within {}: {}", parent, cnt);
}

fn count_all_children(parent: &str, map: &HashMap<String, HashSet<(i32, String)>>) -> i32 {
    if !map.contains_key(parent) {
        return 0;
    }
    let children = map.get(parent).unwrap();
    let mut cnt = 0;
    for c in children {
        cnt += c.0;
        cnt += c.0*count_all_children(&c.1, map); 
    }
    return cnt;
}

fn get_all_parents(child: &str, mut parents: &mut HashSet<String>, map: &HashMap<String, HashSet<String>>) {
    if !map.contains_key(child) {
        return;
    }
    let tmp = map.get(child).unwrap()
        .iter()
        .filter(|x| !parents.contains(*x))
        .collect_vec();
    for &x in tmp.iter() {
        // println!("Add {} from rule {} }} {:?}", x, child, map.get(child).unwrap());
        if x != "no other bag" {
            parents.insert(x.clone());
            get_all_parents(x, &mut parents, map);
        }
    }
}

// Return child_to_parent, parent_to_child
fn load() -> (HashMap<String, HashSet<String>>, HashMap<String, HashSet<(i32, String)>>) {
    let input = tools::load(FNAME);

    let mut child_to_parent: HashMap<String, HashSet<String>> = HashMap::new();
    let mut parent_to_child: HashMap<String, HashSet<(i32, String)>> = HashMap::new();
    
    for line in input {
        let mut parts = line.split("contain");
        let a = parts.next().unwrap().trim().trim_end_matches("s").to_string();
        let b = parts.next().unwrap().trim().trim_end_matches(".");
        let c = b.split(", ").map(|x| x.trim().trim_start_matches(char::is_numeric).trim_end_matches("s").trim().to_string()).collect_vec();
        for x in &c {
            let tmp = child_to_parent.entry(x.clone()).or_insert(HashSet::new());
            tmp.insert(a.clone());
        }
        let c = b.split(", ").filter(|x| *x != "no other bags").map(|x| {
            let mut parts = x.trim().splitn(2, char::is_whitespace);
            let n = parts.next().unwrap().parse().unwrap();
            let v = parts.next().unwrap().trim().trim_end_matches("s").to_string();
            (n, v)
        }).collect_vec();
        let tmp = parent_to_child.entry(a).or_insert(HashSet::new());
        tmp.extend(c);
    }
    return (child_to_parent, parent_to_child);
}