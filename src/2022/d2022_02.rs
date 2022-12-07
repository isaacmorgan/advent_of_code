use crate::tools;

const FNAME: &str = "./input/2022/2022-02-00.txt";

pub fn main() {
    let input = tools::load(FNAME);
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) -> i32 {
    let mut score = 0;
    for ln in input {
        score += match ln.get(2..3) {
            Some("X") => 1,
            Some("Y") => 2,
            Some("Z") => 3,
            _ => 0,
        };

        score += match (ln.get(0..1), ln.get(2..3)) {
            (Some("A"), Some("Y"))
            |(Some("B"), Some("Z"))
            |(Some("C"), Some("X")) => 6,
            (Some("A"), Some("X"))
            |(Some("B"), Some("Y"))
            |(Some("C"), Some("Z")) => 3,
            _ => 0,
        }
    }
    println!("Score: {}", score);
    score
}

fn part2(input: &Vec<String>) -> i32 {
    let mut score = 0;
    for ln in input {
        score += match ln.get(2..3) {
            Some("X") => 0, // Lose
            Some("Y") => 3, // Draw
            Some("Z") => 6, // Win
            _ => 0,
        };

        score += match (ln.get(0..1), ln.get(2..3)) {
            (Some("A"), Some("Y"))
            |(Some("B"), Some("X"))
            |(Some("C"), Some("Z")) => 1, // You should pick Rock
            (Some("A"), Some("Z"))
            |(Some("B"), Some("Y"))
            |(Some("C"), Some("X")) => 2, // You should pick Paper
            (Some("A"), Some("X"))
            |(Some("B"), Some("Z"))
            |(Some("C"), Some("Y")) => 3, // You should pick Scissors
            _ => 0,
        }
    }
    println!("Score: {}", score);
    score
}
