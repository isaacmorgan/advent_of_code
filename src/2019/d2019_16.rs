use std::ops::Rem;
const FNAME: &str = "./input/2019/2019-16.txt";
const FNAME0: &str = "./input/2019/2019-16-0.txt";
const FNAME1: &str = "./input/2019/2019-16-1.txt";

pub fn main() {
    // part 1
    let input = load(FNAME);
    let pattern = vec![0, 1, 0, -1];
    iterate_n(&input, &pattern, 100);

    // part 2
    let input = load_10000(FNAME1);
    println!("{}", input.len());
    iterate_n(&input, &pattern, 100);
}

fn iterate_n(signal: &Vec<i32>, pattern: &Vec<i32>, n: i32) {
    let mut signal = signal.clone();
    for i in 0..n {
        println!("{}", i);
        signal = phase(&signal, &pattern);
        // println!("{:?}", signal);
    }
    print!("Output ({}): ", n);
    print!(" ");
    for i in 0..8 {
        print!("{}", signal.get(i).unwrap());
    }
    //print!(" ");
    //signal.iter().for_each(|x| print!("{}", x));
    println!();
}

fn load(fname: &str) -> Vec<i32> {
    let raw = std::fs::read_to_string(fname).unwrap();
    let input = raw.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    return input;
}

fn load_10000(fname: &str) -> Vec<i32> {
    let raw = std::fs::read_to_string(fname).unwrap();
    let input: Vec<i32> = raw.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut out = Vec::new();
    for i in 0..10_000 {
        out.extend(&input);
        // println!("out: {:?}", out);
    }
    return out;
}

fn phase(input: &Vec<i32>, pattern: &Vec<i32>) -> Vec<i32> {
    let n = input.len();
    let m = pattern.len();
    let mut output = Vec::new();
    for i in 0..n {
        let mut c = 0;
        let mut j = i;
        //let mut c = calc_digit(&input, &pattern, i as i32);
        //for j in i..n {
        while j < n {
            let p_i = ((j+1)/(i+1)).rem_euclid(m);
            if pattern.get(p_i).unwrap() == &0 {
                j += (i+1);
                continue;
            }
            let a = input.get(j).unwrap();
            let b = pattern.get(p_i).unwrap();
            c += a*b;
            j += 1;
            // print!("{}*{} ({}) + ", a, b, c);
        }
        
        c = c.rem(10).abs();
        // print!(" = {}", c);
        // println!();
        output.push(c);
    }
    return output;
}

fn calc_digit(input: &Vec<i32>, pattern: &Vec<i32>, ind: i32) -> i32 {
    //let m = pattern.len()*(ind as usize); // Length of expanded pattern
    let n = ind + 1;
    let mut s = 0;
    let mut i = ind as usize;
    let mut j = 1;
    while i < input.len() {
        match j {
            0 | 2 => {
                i += 1;
                j += 1;
            },
            1 => {
                for k in 0..n {
                    s += input.get(i).unwrap_or(&0);
                    i += 1;
                }
                j += 1;
            },
            3 => {
                for k in 0..n {
                    s -= input.get(i).unwrap_or(&0);
                    i += 1;
                }
                j = 0;
            },
            _ => (),
        }
    }
    return s;
}
