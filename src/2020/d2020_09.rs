use crate::tools;
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-09.txt";
static N:usize = 25;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let a = load();
    let v = first_invalid_num(&a);
    println!("Invalid Number: {}", v);
}

fn part2() {
    let a = load();
    let v = first_invalid_num(&a);
    if let Some((i, j)) = contiguous_sum(&a, v) {
        let ans = a[i..=j].iter().min().unwrap() + a[i..=j].iter().max().unwrap();
        println!("Answer: {}", ans);
    }
    
}

fn contiguous_sum(list: &Vec<i64>, n: i64) -> Option<(usize, usize)> {
    let mut min = 0;
    let mut max = 0;
    let mut sum = list[0];
    loop {
        if sum > n {
            sum -= list[min];
            min += 1;
        } else if sum < n {
            max += 1;
            sum += list[max];
        } else {
            return Some((min, max));
        }
    }
}

fn first_invalid_num(list: &Vec<i64>) -> i64 {
    'outer: for i in N..list.len() {
        for j in i-N..i {
            for k in j+1..i {
                if list[j] + list[k] == list[i] {
                    continue 'outer;
                }
            }
        }
        return list[i];
    }
    return -1;
}

fn load() -> Vec<i64> {
    let input = tools::load(FNAME);
    input.iter().map(|x| x.parse().unwrap()).collect_vec()
}