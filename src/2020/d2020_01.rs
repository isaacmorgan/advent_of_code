use crate::tools;
use itertools::Itertools;

const FNAME: &str = "./input/2020/2020-01.txt";

pub fn main() {
    let input = load(FNAME);
    if let Some(res) = part1(&input, 2020) {
        println!("Entries: {} {} Multiply: {}", res.0, res.1, res.2);
    }
    if let Some(res) = part2(&input, 2020) {
        println!("Entries: {} {} {} Multiply: {}", res.0, res.1, res.2, res.3);
    }
}

fn part1(input: &Vec<i32>, n: i32) -> Option<(i32, i32, i32)> {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            if input[i] + input[j] == n {
                return Some((input[i], input[j], input[i]*input[j]));
            }
        }
    }
    return None;
}

fn part2(input: &Vec<i32>, n: i32) -> Option<(i32, i32, i32, i32)> {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            for k in j+1..input.len() {
                if input[i] + input[j] + input[k] == n {
                    return Some((input[i], input[j], input[k], input[i]*input[j]*input[k]));
                } 
            }
        }
    }
    return None;
}

fn load(fname: &str) -> Vec<i32> {
    let input = tools::load(fname);
    input.iter().map(|s| s.parse::<i32>().unwrap()).collect_vec()
}

