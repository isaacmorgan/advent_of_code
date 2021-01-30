use crate::tools;
use itertools::{Itertools, enumerate};

static INPUT1: &str = "389125467";
static INPUT: &str = "157623984";

#[derive(Debug)]
struct Game {
    p: usize,
    len: usize,
    val: Vec<usize>,
    next: Vec<usize>,
    prev: Vec<usize>,
    valid: Vec<bool>,
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let mut game = load(INPUT1, 9);
    println!("{:?}", game);
    print_game(&game);
    for _ in 0..100 {
        do_move(&mut game);
        // print_game(&game);
    }
    print_labels(&game);
}

fn part2() {
    let mut game = load(INPUT, 1_000_000);
    // println!("{:?}", game);
    // print_game(&game);
    for _ in 0..10_000_000 {
        do_move(&mut game);
        // print_game(&game);
    }
    // print_labels(&game);
    let mut p = game.next[1];
    let l1 = game.val[p];
    p = game.next[p];
    let l2 = game.val[p];
    println!("l1: {} l2: {} label_product: {}", l1, l2, l1*l2);
}

fn print_labels(game: &Game) {
    print!("Labels: ");
    let mut p = 1;
    for _ in 0..game.len-2 {
        p = game.next[p];
        print!("{}", game.val[p]);
    }
    println!();
}

fn print_game(game: &Game) {
    print!("cups: ");
    let mut p = game.p;
    for _ in 0..game.len-1 {
        print!("{} ", game.val[p]);
        p = game.next[p];
    }
    println!();
}

fn do_move(game: &mut Game) {
    let old_next = do_pickup(game);
    let destination = do_destination_cup(game);
    do_place(game, destination, old_next);
    do_select(game);
}

fn do_pickup(game: &mut Game) -> usize {
    let mut p = game.p;
    let old_next = game.next[p]; 
    for _ in 0..3 {
        p = game.next[p];
        game.valid[p] = false;
    }
    game.next[game.p] = game.next[p];
    game.prev[game.next[game.p]] = game.p;
    return old_next;
}

fn do_destination_cup(game: &Game) -> usize {
    let mut label = game.val[game.p];
    label = label - 1;
    while !game.valid[label] {
        label = (label as i32 - 1).rem_euclid(game.len as i32) as usize;
    }
    return label;
}

fn do_place(game: &mut Game, destination: usize, old_next: usize) {
    let new_next = game.next[destination];
    game.next[destination] = old_next;
    game.prev[old_next] = destination;
    let mut p = old_next;
    game.valid[p] = true;
    for _ in 0..2 {
        p = game.next[p];
        game.valid[p] = true;
    }
    game.next[p] = new_next;
    game.prev[new_next] = p;
}

fn do_select(game: &mut Game) {
    game.p = game.next[game.p];
}

fn load(input: &str, len: usize) -> Game {
    let input_usize = input.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec();
    let l = input.len();
    let tmp = (0..(len+1)).collect_vec();
    let mut next = tmp.clone(); // vec![0; len + 1];
    next.remove(1);
    next.push(1);
    let mut prev = tmp.clone(); //vec![0; len + 1];
    prev.insert(1, len);
    prev.pop();
    let mut val = tmp.clone();
    let mut valid = vec![true; len + 1];
    valid[0] = false;
    // prev[*input_usize.first().unwrap()] = *input_usize.last().unwrap();
    // next[*input_usize.first().unwrap()] = input_usize[1];
    for (i, &c) in enumerate(&input_usize) {
        prev[c] = input_usize[((i as i32-1).rem_euclid(l as i32)) as usize].clone();
        next[c] = input_usize[(i+1)%l];
        val[c] = c;
        if i == 0 && len > 9 {
            prev[c] = len;
            next[len] = c;
        } else if i == l - 1 && len > 9 {
            next[c] = l + 1;
            prev[l + 1] = c;
        }
    }
    
    Game{
        p: *input_usize.first().unwrap(),
        len: len + 1,
        val,
        next,
        prev,
        valid,
    }
}