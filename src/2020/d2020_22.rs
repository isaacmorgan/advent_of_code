use crate::tools;
use std::collections::HashSet;

static FNAME: &str = "./input/2020/2020-22.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut d1, mut d2) = load();
    println!("{:?}", d1);
    println!("{:?}", d2);
    loop {
        step(&mut d1, &mut d2);
        if d1.len() == 0 || d2.len() == 0 {
            break;
        }
    }
    println!("d1: {:?}", d1);
    println!("d2: {:?}", d2);
    let winner = if d1.len() == 0 { &d2 } else { &d1 };
    println!("Winning Score: {}", score(winner));
}

fn part2() {
    let (mut d1, mut d2) = load();
    println!("{:?}", d1);
    println!("{:?}", d2);
    let winner = if game_loop_r(&mut d1, &mut d2) {
        &d1
    } else {
        &d2
    };
    println!("d1: {:?}", d1);
    println!("d2: {:?}", d2);
    println!("Winning Score: {}", score(winner));
}

fn game_loop_r(a: &mut Vec<i32>, b: &mut Vec<i32>) -> bool {
    // println!("New Game:");
    let mut did_a_win = false;
    let mut hist = HashSet::new();
    loop {
        if !step_r(a, b, &mut hist) {
            did_a_win = true;
            break;
        }
        if a.len() == 0 {
            did_a_win = false;
            break;
        } else if b.len() == 0 {
            did_a_win = true;
            break;
        }
    }
    return did_a_win;
}

fn step_r(a: &mut Vec<i32>, b: &mut Vec<i32>, hist: &mut HashSet<(Vec<i32>, Vec<i32>)>) -> bool {
    // Return false if a wins by history loop, otherwise return true
    // println!("New Round:");
    // println!("d1: {:?}\nd2: {:?}", a, b);
    if hist.contains(&(a.clone(), b.clone())) {
        // println!("d1 wins game by history loop");
        return false; // Player a wins game
    } else {
        hist.insert((a.clone(), b.clone()));
    }
    let ca = a.remove(0);
    let cb = b.remove(0);
    let did_a_win = if ca <= a.len() as i32 && cb <= b.len() as i32 {
        game_loop_r(&mut a[0..ca as usize].to_vec(), &mut b[0..cb as usize].to_vec())
    } else {
        if ca > cb {
            true
        } else {
            false
        }
    };
    if did_a_win {
        // println!("d1 wins!");
        push_winner(a, ca, cb);
    } else {
        // println!("d2 wins!");
        push_winner(b, cb, ca);
    }
    return true;
}

fn score(deck: &Vec<i32>) -> i32 {
    deck.iter().rev().enumerate().map(|(i, v)| (i as i32 + 1)*v).sum()
}

fn step(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    let ca = a.remove(0);
    let cb = b.remove(0);
    if ca > cb {
        push_winner(a, ca, cb);
    } else {
        push_winner(b, cb, ca);
    }
}

fn push_winner(deck: &mut Vec<i32>, card_win: i32, card_lose: i32) {
    deck.push(card_win);
    deck.push(card_lose);
}

fn load() -> (Vec<i32>, Vec<i32>) {
    let input = tools::load(FNAME);
    let mut player = 0;
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();
    for line in input {
        match line.as_str() {
            "Player 1:" => player = 1,
            "Player 2:" => player = 2,
            "" => (),
            _ => {
                if player == 1 {
                    deck1.push(line.parse().unwrap());
                } else {
                    deck2.push(line.parse().unwrap());
                }
            }
        }
    }
    return (deck1, deck2);
}