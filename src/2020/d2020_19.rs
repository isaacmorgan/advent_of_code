use crate::tools;
use std::collections::HashMap;

static FNAME: &str = "./input/2020/2020-19.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (rulebook,messages) = load();
    println!("{:?}", rulebook);
    println!("{:?}", messages);
    let mut cnt = 0;
    for msg in messages {
        let q = validate_part(&rulebook, "0", &msg);
        println!("{} : {}", msg, !q.is_empty() && q.first().unwrap().is_empty());
        if !q.is_empty() && q.first().unwrap().is_empty() {
            cnt += 1;
        }
        // println!("q: {:?}", q);
    }
    println!("Valid Count: {}", cnt);
}

fn part2() {
    let (mut rulebook,messages) = load();
    rulebook.insert("8".to_string(), "42 | 42 8".to_string());
    rulebook.insert("11".to_string(), "42 31 | 42 11 31".to_string());
    println!("{:?}", rulebook);
    println!("{:?}", messages);
    let mut cnt = 0;
    for msg in messages {
        let q = validate_part(&rulebook, "0", &msg);
        println!("{} : {}", msg, !q.is_empty() && q.first().unwrap().is_empty());
        if !q.is_empty() && q.first().unwrap().is_empty() {
            cnt += 1;
        }
        // println!("q: {:?}", q);
    }
    println!("Valid Count: {}", cnt);
}

fn validate_part(rulebook: &HashMap<String, String>, rule: &str, msg: &str) -> Vec<String> {
    let mut out = Vec::new(); 
    let branch = rule.split(" | ");
    // println!("msg: {}", msg);
    for b in branch {
        // println!("b: {}", b);
        // End of line, return remainder
        if b.chars().next().unwrap().is_alphabetic() {
            return if msg.starts_with(b) {
                // Found match, return substring
                vec![msg[b.len()..].to_string()]
            } else {
                out
            }
        }
        
        // Check each part of branch and find remainder
        let mut mopts = vec![msg.to_string()];
        for s in b.split(" ") {
            let mut next_mopts = Vec::new();
            for m in mopts {
                // println!("s: {}", s);
                let rem = validate_part(rulebook, rulebook.get(s).unwrap(), &m);
                for r in rem {
                    next_mopts.push(r);
                }
            }
            mopts = next_mopts;
        }
        out.extend(mopts);
    }
    return out;
}

fn load() -> (HashMap<String, String>, Vec<String>) {
    let input = tools::load(FNAME);
    let mut mode = 0;
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    for line in input {
        if line.is_empty() {
            mode += 1;
            continue;
        }
        if mode == 0 {
            let mut s = line.split(": ");
            let a = s.next().unwrap().to_string();
            let b = s.next().unwrap().trim_start_matches("\"").trim_end_matches("\"").to_string();
            rules.insert(a, b);
        } else {
            messages.push(line);
        }
    }
    
    return(rules, messages);
}