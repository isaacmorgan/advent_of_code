use std::collections::HashSet;
use crate::tools;
static FNAME: &str = "./input/2022/2022-03-01.txt";

pub fn main() {
    let input = load();
    part1(&input);
    part2(&input);
}

fn part1(lines: &Vec<String>) {
    let mut priority_sum = 0;
    for ln in lines {
        let x = item_type_to_priority(ln[0..1].chars().next().unwrap());
        let (a,b) = split_item_bins(&ln);
        let dup = duplicate_item_types(a, b);
        priority_sum += item_type_to_priority(dup);
    }
    println!("Priority sum 1: {priority_sum}");
}

fn part2(lines: &Vec<String>) {
    let mut priority_sum = 0;
    for ((a,b), c) in lines.iter().step_by(3)
        .zip(lines.iter().skip(1).step_by(3))
        .zip(lines.iter().skip(2).step_by(3)){
        let intersect = string_intersection(&string_intersection(a, b), c);
        priority_sum += item_type_to_priority(intersect.chars().next().unwrap());
    }
    println!("Priority sum 2: {priority_sum}");
}

fn split_item_bins(item_list: &str) -> (&str, &str) {
    let l = item_list.len()/2;
    return (&item_list[0..l], &item_list[l..]);
}

fn string_intersection(a: &str, b: &str) -> String {
    let mut out = Vec::new();
    for ca in a.chars() {
        for cb in b.chars() {
            if ca == cb {
                out.push(cb);
            }
        }
    }
    return out.iter().collect::<String>();
}

fn duplicate_item_types(a: &str, b: &str) -> char {
    for ca in a.chars() {
        for cb in b.chars() {
            if ca == cb {
                return cb;
            }
        }
    }
    panic!("No character match found between {} and {}", a, b);
}

fn item_type_to_priority(it: char) -> i32 {
    match it {
        'a'..='z' => return it as i32 - 96,
        'A'..='Z' => return it as i32 - 64 + 26,
        _ => panic!("Unknown item type: {}", it),
    }
}

fn load() -> Vec<String> {
    return tools::load(FNAME);
}