use std::time::{Instant};
use mod_exp::mod_exp;

const FNAME: &str = "./input/2019/2019-22.txt";
const FNAME0: &str = "./input/2019/2019-22-3.txt";

#[derive(Debug, Clone)]
enum Tech {
    Null,
    DealNew,
    DealInc(i32),
    Cut(i32),
}

pub fn main() {
    let now = Instant::now();
    part_1();
    let then = Instant::now();
    println!("{:?}", then-now);

    let now = Instant::now();
    part_1_2();
    let then = Instant::now();
    println!("{:?}", then-now);
    
    part_2();
}

fn part_2() {
    let mut shuffle = load(FNAME);
    shuffle.reverse();
    
    let L = 119315717514047;
    let N = 101741582076661; 
    // let L = 10007;
    // let N = 1;
    
    let (mul, add) = calculate_rev_shuffle(&shuffle, L);
    
    // fn(ax*b) n times = a^n*x + b*(a^n-1)/(a-1)
    let x = 2020;

    dbg!(mul, add);
    
    // Stole this from somebody online. Still don't understand, something about multiplicative inverse
    // Hope 2020 doesn't have any awful problems like this.
    let term1 = 2020 * mod_exp(mul,N,L) % L;
    let tmp = (mod_exp(mul,N,L) - 1) * mod_exp(mul-1, L-2, L) % L;
    let term2 = add * tmp % L;
    let q = (term1 + term2).rem_euclid(L);
    println!("Card at position {} is {}", x, q);
}

fn part_1_2() {
    let mut shuffle = load(FNAME);
    //shuffle = shuffle[..1].to_vec();
    
    let deck: Vec<i32> = (0..10007).collect();

    let (mut mul, mut add) = calculate_shuffle(&shuffle, deck.len() as i128);
    let new_deck = shuffle_deck(mul, add, &deck);
    for i in 0..new_deck.len() {
        if new_deck[i] == 2019 {
            println!("Card 2019 is at position {}", i);
        }
    }
    let i = 6326;
    println!("Card at position {} is {}", i, new_deck[i]);
}

fn part_1() {
    let shuffle = load(FNAME);
    // println!("{:?}", shuffle);
    let mut deck = (0..10007).collect();
    // println!("{:?}", deck);
    for tech in &shuffle {
        //println!("{:?}", tech);
        step(tech, &mut deck);
        //println!("{:?}", deck);
    }
    for i in 0..deck.len() {
        if deck[i] == 2019 {
            println!("Card 2019 is at position {}", i);
        }
    }
}

fn shuffle_deck(mul: i128, add: i128, deck: &Vec<i32>) -> Vec<i32> {
    let mut out = deck.clone();
    let dlen = deck.len() as i128;
    for &d in deck {
        let i = (d as i128 * mul + add).rem_euclid(dlen);
        out[i as usize] = d;
    }
    return out;
}


fn calculate_rev_shuffle(shuffle: &Vec<Tech>, deck_len: i128) -> (i128, i128) {
    let mut mul = 1;
    let mut add = 0;
    for tech in shuffle {
        let (m, a) = step_rev_mul_add(&tech);
        add = m * add + a;
        mul = m * mul;
        mul = mul.rem_euclid(deck_len);
        add = add.rem_euclid(deck_len);
        // println!("{:?}", tech);
        // println!("({}, {}) : ({}, {})", m, a, mul, add);
    }
    return (mul, add);
}

fn calculate_shuffle(shuffle: &Vec<Tech>, deck_len: i128) -> (i128, i128) {
    let mut mul = 1;
    let mut add = 0;
    for tech in shuffle {
        let (m, a) = step_mul_add(&tech);
        add = m * add + a;
        mul = mul * m;
        mul = mul.rem_euclid(deck_len);
        add = add.rem_euclid(deck_len);
        // println!("{:?}", tech);
        // println!("({}, {}) : ({}, {})", m, a, mul, add);
    }
    return (mul, add);
}

fn step_rev_mul_add(tech: &Tech) -> (i128, i128) {
    match tech {
        Tech::Null => (0, 0),
        Tech::DealNew => (-1, -1),
        Tech::DealInc(v) => (mod_exp(*v as i128, 119315717514047-2, 119315717514047), 0), // mod_inv implements on primes as mod_exp, this is worst part
        //Tech::DealInc(v) => (mod_exp(*v as i128, 10007-2, 10007), 0), // mod_inv implements on primes as mod_exp, this is worst part
        Tech::Cut(v) => (1, *v as i128),
    }
}

fn step_mul_add(tech: &Tech) -> (i128, i128) {
    match tech {
        Tech::Null => (0, 0),
        Tech::DealNew => (-1, -1),
        Tech::DealInc(v) => (*v as i128, 0),
        Tech::Cut(v) => (1, -*v as i128),
    }
}

fn step(tech: &Tech, deck: &mut Vec<i32>) {
    match tech {
        Tech::Null => (),
        Tech::DealNew => deck.reverse(),
        Tech::DealInc(v) => {
            let old = deck.clone();
            for i in 0..old.len() {
                deck[(i * (*v as usize)) % old.len()] = old[i];
            }
        }
        Tech::Cut(v) => {
            if v > &0 {
                deck.rotate_left(*v as usize);
            } else if v < &0 {
                deck.rotate_right(v.abs() as usize);
            }
        }
    }
}

fn load(fname: &str) -> Vec<Tech> {
    let input = std::fs::read_to_string(fname).unwrap();
    let mut shuffle = Vec::new();
    for i in input.lines() {
        let tech = if i.starts_with("deal into new stack") {
            Tech::DealNew
        } else if i.starts_with("deal with increment") {
            Tech::DealInc(i[20..].parse().unwrap())
        } else if i.starts_with("cut") {
            Tech::Cut(i[4..].parse().unwrap())
        } else {
            Tech::Null
        };
        shuffle.push(tech);
    }
    return shuffle;
}
