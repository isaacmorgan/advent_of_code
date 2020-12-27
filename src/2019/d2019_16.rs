use std::ops::Rem;

const FNAME: &str = "./input/2019/2019-16.txt";
const FNAME0: &str = "./input/2019/2019-16-0.txt";
const FNAME1: &str = "./input/2019/2019-16-1.txt";
const FNAME4: &str = "./input/2019/2019-16-4.txt";

pub fn main() {
    // part 1
    let input = load(FNAME);
    let pattern = vec![0, 1, 0, -1];
    iterate_n(&input, &pattern, 100, false);

    // part 2
    let input = load_10000(FNAME);
    println!("Input length: {}", input.len());
    iterate_n(&input, &pattern, 100, true);
}

fn iterate_n(signal: &Vec<i32>, pattern: &Vec<i32>, n: i32, use_offset: bool) {
    let mut offset = 0;
    if use_offset {
        for i in 0..7 {
            offset *= 10;
            offset += signal.get(i).unwrap();
        }
    }

    println!("Offset: {}", offset);

    let mut signal = signal[(offset as usize)..].to_vec();
    println!("signal length: {:?}", signal.len());

    if use_offset {
        for _i in 0..n {
            // println!("{}", i);
            signal = phase_offset_approx(&signal);
            // println!("sig: {:?}", signal);
        }
    } else {
        for _i in 0..n {
            // println!("{}", i);
            signal = phase(&signal, &pattern, offset as usize);
            // println!("{:?}", signal);
        }
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
    for _i in 0..10_000 {
        out.extend(&input);
        // println!("out: {:?}", out);
    }
    return out;
}

fn phase_offset_approx(input: &Vec<i32>) -> Vec<i32> {
    // Assume that the offset is large enough that the calculation of each digit is such that
    // d_0 = d_1 + previous d_0
    // Then work backwards starting from d_end and ending at d_offset
    let mut output = input.clone();
    for i in (0..(input.len()-1)).rev() {
        output[i] = (output[i] + output[i+1]).rem(10);
    }
    return output;
}

fn phase(input: &Vec<i32>, pattern: &Vec<i32>, offset: usize) -> Vec<i32> {
    let n = input.len();
    let m = pattern.len();
    let mut output = input.clone();
    let mut c;
    let mut p_i;
    for i in 0..n {
        c = 0;
        let mut j = i;
        while j < n {
        // for j in i..n {
            p_i = ((j+1+offset)/(i+1+offset)).rem(m);
            c += input[j]*pattern[p_i];
            // print!("{}*{} ({}) + ", a, b, c);
            if pattern[p_i] == 0 && pattern[(p_i as i32 - 1).rem_euclid(m as i32) as usize] != 0 {
                j += i+1;
            } else {
                j += 1;
            }
        }
        c = c.rem(10).abs();
        // print!(" = {}", c);
        // println!();
        output[i] = c;
    }
    return output;
}
