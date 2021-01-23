use crate::tools;
use std::collections::{HashMap, HashSet};
use itertools::{Itertools, enumerate};
use std::iter::FromIterator;

static FNAME: &str = "./input/2020/2020-16.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (rules, my_ticket, tickets) = load();
    println!("{:?}", rules);
    println!("{:?}", my_ticket);
    println!("{:?}", tickets);
    let tser = ticket_scanning_error_rate(&tickets, &rules);
    println!("TSER: {}", tser);
}

fn part2() {
    let (rules, my_ticket, tickets) = load();
    let mut vr = valid_rules(&my_ticket, &tickets, &rules);
    println!("{:?}", vr);
    while vr.iter().filter(|x| x.len() > 1).count() > 0 {
        solve_rules(&mut vr);
    }
    println!("{:?}", vr);
    let mut dfields = Vec::new();
    for (i, r) in enumerate(rules) {
        if r.0.starts_with("departure") {
            dfields.push(i);
        }
    }
    println!("dfields: {:?}", dfields);
    let mut res: i64 = 1;
    for (j, h) in enumerate(vr) {
        let i = h.iter().next().unwrap().clone();
        if dfields.contains(&i) {
            res *= my_ticket[j] as i64;
            println!("{}", my_ticket[j]);
        }
    }
    println!("Ticket Multiply: {}", res);
}

fn solve_rules(rule_inds: &mut Vec<HashSet<usize>>) {
    // remove_single_rules(rule_inds);
    remove_solitary_rules(rule_inds);
}

fn remove_solitary_rules(rule_inds: &mut Vec<HashSet<usize>>) {
    for i in 0..rule_inds.len() {
        let mut cnt = 0;
        for r in rule_inds.iter() {
            if r.contains(&i) {
                cnt += 1;
            }
        }
        
        if cnt == 1 {
            for r in rule_inds.iter_mut() {
                if r.contains(&i) {
                    r.clear();
                    r.insert(i);
                    // println!("{}", i);
                }
            }
        }
    }
}

fn remove_single_rules(rule_inds: &mut Vec<HashSet<usize>>) {
    let mut finds = Vec::new();
    for (i, r) in enumerate(rule_inds.iter()) {
        if r.len() == 1 {
            finds.push((i, r.iter().next().unwrap().clone()));
        }
    }
    for (i, f) in finds {
        for r in rule_inds.iter_mut() {
            if r.len() > 0 && r.contains(&f) {
                r.remove(&f);
            }
        }
    }
}

fn valid_rules(my_ticket: &Vec<i32>, tickets: &Vec<Vec<i32>>, rules: &Vec<(String, Vec<i32>, Vec<i32>)>) -> Vec<HashSet<usize>> {
    let mut valid_rules = Vec::new();
    for (i, v) in enumerate(my_ticket) {
        valid_rules.push(valid_rule_inds(*v, rules));
    }
    // println!("{:?}", valid_rules);
    
    let valid_tickets = tickets.iter().filter(|x| is_ticket_valid(*x, rules)).collect_vec();
    for t in valid_tickets {
        for (i, v) in enumerate(t) {
            let inds = valid_rule_inds(*v, rules);
            valid_rules[i] =  HashSet::from_iter(valid_rules[i].intersection(&inds).map(|x| *x).collect_vec());
        }
    }
    return valid_rules;
    // println!("{:?}", valid_rules);
}

fn valid_rule_inds(v: i32, rules: &Vec<(String, Vec<i32>, Vec<i32>)>) -> HashSet<usize> {
    let mut out = HashSet::new();
    for (i, (_, r1, r2)) in enumerate(rules) {
        if (v >= r1[0] && v <= r1[1])
            || (v >= r2[0] && v <= r2[1]) {
            out.insert(i);
        }
    }
    return out;
}

fn ticket_scanning_error_rate(tickets: &Vec<Vec<i32>>, rules: &Vec<(String, Vec<i32>, Vec<i32>)>) -> i32 {
    tickets.iter().flatten().filter(|x| !is_valid(**x, rules)).sum()
}

fn is_ticket_valid(ticket: &Vec<i32>, rules: &Vec<(String, Vec<i32>, Vec<i32>)>) -> bool {
    for t in ticket {
        if !is_valid(*t, rules) {
            return false;
        }
    }
    return true;
}

fn is_valid(value: i32, rules: &Vec<(String, Vec<i32>, Vec<i32>)>) -> bool {
    for (_, r1, r2) in rules {
        if (value >= r1[0] && value <= r1[1]) 
            || (value >= r2[0] && value <= r2[1]) {
            return true;
        } 
    }
    return false;
}

fn load() -> (Vec<(String, Vec<i32>, Vec<i32>)>, Vec<i32>, Vec<Vec<i32>>) {
    let input = tools::load(FNAME);
    
    let mut rules = Vec::new();
    let mut my_ticket: Vec<i32> = Vec::new();
    let mut tickets = Vec::new();
    
    let mut mode = 0;
    for line in &input {
        if line.is_empty() {
            mode += 1;
            continue;
        }
        match mode {
            0 => {
                rules.push(parse_rule(line));
            }
            1 => {
                if line.starts_with("your ticket:") {
                    continue;
                }
                my_ticket.extend(line.split(",").map(|x| x.parse::<i32>().unwrap()));
            }
            _ => {
                if line.starts_with("nearby tickets:") {
                    continue;
                }
                tickets.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect_vec());
            }
        }
    }
    
    return (rules, my_ticket, tickets);
}

fn parse_rule(s: &str) -> (String, Vec<i32>, Vec<i32>){
    let mut parts = s.split(": ");
    let name = parts.next().unwrap().to_string();
    parts = parts.next().unwrap().split(" or ");
    let rule_1 = parts.next().unwrap().split("-").map(|x| x.parse().unwrap()).collect_vec();
    let rule_2 = parts.next().unwrap().split("-").map(|x| x.parse().unwrap()).collect_vec();
    return (name, rule_1, rule_2)
}