use crate::tools;
use itertools::{enumerate, PeekingNext};

static FNAME: &str = "./input/2020/2020-18.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let input = load();
    let mut sum = 0;
    for i in input {
        // println!("{:?}", i);
        let v = eval(&i);
        // println!(" = {}", v);
        sum += v;
    }
    
    println!("Sum: {}", sum);
}

fn part2() {
    let input = load();
    let mut sum = 0;
    for i in input {
        // println!("{:?}", i);
        let v = eval2(&i);
        // println!(" = {}", v);
        sum += v;
    }

    println!("Sum: {}", sum);
}

fn load() -> Vec<String> {
    tools::load(FNAME)
}

fn eval(s: &str) -> i64 {
    let mut i0 = 0;
    let mut ind = 0;
    let mut cmds = vec!["+".to_string()];
    for (i1, c) in enumerate(s.chars()) {
        if ind == 0 && c == ' '  {
            cmds.push(s[i0..i1].to_string());
            i0 = i1 + 1;
        } else if c == '(' {
            ind += 1;
        } else if c == ')' {
            ind -= 1;
        }
    }
    cmds.push(s[i0..].to_string());
    // println!("{:?}", cmds);
    
    let mut v = 0;
    let mut citer = cmds.iter();
    // for a in citer.step_by(2) {
    for _ in (0..citer.len()).step_by(2) {
        let a = citer.next().unwrap();
        let b = citer.next().unwrap();
        let c = if b.starts_with('(') {
            eval(&b[1..b.len()-1])
        } else {
            b.parse::<i64>().unwrap()
        };
        match a.as_str() {
            "*" => v *= c,
            "+" => v += c,
            _ => panic!("Bad"),
        }
    }
    return v;
}


fn eval2(s: &str) -> i64 {
    let mut i0 = 0;
    let mut ind = 0;
    let mut cmds = vec!["+".to_string()];
    for (i1, c) in enumerate(s.chars()) {
        if ind == 0 && c == ' '  {
            cmds.push(s[i0..i1].to_string());
            i0 = i1 + 1;
        } else if c == '(' {
            ind += 1;
        } else if c == ')' {
            ind -= 1;
        }
    }
    cmds.push(s[i0..].to_string());
    // println!("{:?}", cmds);
    
    // If there are any '+' then replace all 'x' '*' 'y' with '(x * y)'
    if cmds.contains(&"*".to_string()) {
        let mut xind = Vec::new();
        for (i, c) in enumerate(&cmds) {
            if i > 0 && c == "+" {
                xind.push((i-1, ["(".to_string(), cmds[i-1..=i+1].join(" "), ")".to_string()].join("")));
            }
        }
        for (j, (i, c)) in enumerate(xind) {
            let mut n = String::from("(");
            n.push_str(&cmds.remove(i - j*2));
            n.push_str(" ");
            n.push_str(&cmds.remove(i - j*2));
            n.push_str(" ");
            n.push_str(&cmds.remove(i - j*2));
            n.push_str(")");
            cmds.insert(i - j*2, n)
        }
        // println!("{:?}", cmds);
    }

    let mut v = 0;
    let mut citer = cmds.iter();
    // for a in citer.step_by(2) {
    for _ in (0..citer.len()).step_by(2) {
        let a = citer.next().unwrap();
        let b = citer.next().unwrap();
        let c = if b.starts_with('(') {
            eval2(&b[1..b.len()-1])
        } else {
            b.parse::<i64>().unwrap()
        };
        match a.as_str() {
            "*" => v *= c,
            "+" => v += c,
            _ => panic!("Bad"),
        }
    }
    return v;
}
