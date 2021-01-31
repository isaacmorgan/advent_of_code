use crate::tools;
use itertools::Itertools;

static FNAME: &str = "./input/2020/2020-25.txt";
static MOD_NUM: i64 = 20201227;
static SUBJ_NUM: i64 = 7;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let pub_key = load();
    println!("{:?}", pub_key);
    let loop_size_a= find_loop_size(SUBJ_NUM, pub_key[0]);
    let loop_size_b= find_loop_size(SUBJ_NUM, pub_key[1]);
    let encryption_key_a = transform(pub_key[1], loop_size_a);
    let encryption_key_b = transform(pub_key[0], loop_size_b);
    if encryption_key_a == encryption_key_b {
        println!("Encryption Key: {}", encryption_key_a);
    } else {
        println!("Encryption key not found.\nLoop size a: {} Loop size b: {}", loop_size_a, loop_size_b);
    }
}

fn part2() {

}

fn find_loop_size(subj_num: i64, pub_key: i64) -> u32 {
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        loop_size += 1;
        value = (value*subj_num).rem_euclid(MOD_NUM);
        if value == pub_key {
            break;
        }
    }
    return loop_size;
}

fn transform(subject_number: i64, loop_size: u32) -> i64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value*subject_number).rem_euclid(MOD_NUM);
    }
    return value;
}

fn load() -> Vec<i64> {
    let input = tools::load(FNAME);
    input.iter().map(|x| x.parse().unwrap()).collect_vec()
}