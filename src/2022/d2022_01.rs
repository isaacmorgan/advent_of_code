use crate::tools;
use itertools::Itertools;

const FNAME: &str = "./input/2022/2022-01-00.txt";

pub fn main() {
    let input = load(FNAME);
    let ans1 = part1(&input);
    println!("Max Calories: {}", ans1);

    let ans2 = part2(&input);
    println!("Max Calories: {}", ans2);
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max = 0;
    for i in input {
        if i > &0 {
            sum += i;
        } else {
            if sum > max {
                max = sum;
                dbg!(max);
            }
            sum = 0;
        }
    }
    return max;
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut cals = Vec::new();
    let mut sum = 0;
    for i in input {
        if i > &0 {
            sum += i;
        } else {
            cals.push(sum);
            sum = 0;
        }
    }
    
    cals.sort();
    cals.reverse();
    dbg!(&cals);
    return cals[..3].iter().sum();
}

fn load(fname: &str) -> Vec<i32> {
    let input = tools::load(fname);
    input.iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect_vec()
}

