use crate::tools;
use itertools::Itertools;
use std::fmt;

static FNAME: &str = "./input/2020/2020-12.txt";

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    r: i32,
}

impl Ship {
    fn step(&mut self, action: &Action) {
        match action {
            Action::N(i) => self.y += i,
            Action::S(i) => self.y -= i,
            Action::E(i) => self.x += i,
            Action::W(i) => self.x -= i,
            Action::L(i) => self.r -= i,
            Action::R(i) => self.r += i,
            Action::F(i) => self.forward(i.clone()),
            Action::None(i) => (),
        }
    }
    
    fn forward(&mut self, i: i32) {
        match self.r.rem_euclid(360) {
            0 => self.step(&Action::E(i)),
            90 => self.step(&Action::S(i)),
            180 => self.step(&Action::W(i)),
            270 => self.step(&Action::N(i)),
            _ => println!("Invalid Angle: {}", self.r),
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn step(&mut self, action: &Action) {
        match action {
            Action::N(i) => self.y += i,
            Action::S(i) => self.y -= i,
            Action::E(i) => self.x += i,
            Action::W(i) => self.x -= i,
            Action::L(i) => {
                for j in 0..i/90 {
                    let x = -self.y;
                    let y = self.x;
                    self.x = x;
                    self.y = y;
                }
            },
            Action::R(i) => {
                for j in 0..i/90 {
                    let x = self.y;
                    let y = -self.x;
                    self.x = x;
                    self.y = y;
                }
            },
            Action::F(i) => (),
            Action::None(i) => (),
        }
    }
}

#[derive(Debug)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
    None(i32),
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::N(i) => write!(f, "N({})", i),
            Action::S(i) => write!(f, "S({})", i),
            Action::E(i) => write!(f, "E({})", i),
            Action::W(i) => write!(f, "W({})", i),
            Action::L(i) => write!(f, "L({})", i),
            Action::R(i) => write!(f, "R({})", i),
            Action::F(i) => write!(f, "F({})", i),
            Action::None(i) => write!(f, "None"),
        }
    }
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let actions = load();
    // println!("{:?}", actions);
    let mut ship = Ship {
        x: 0,
        y: 0,
        r: 0
    };
    for a in actions {
      ship.step(&a);
        // println!("{:} => {:?}", a, ship);
    }
    println!("Distance Travelled: x: {} y: {} sum: {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}

fn part2() {
    let actions = load();
    let mut ship = Ship {
        x: 0,
        y: 0,
        r: 0
    };
    let mut waypoint = Waypoint { x: 10, y: 1 };
    for a in actions {
        if let Action::F(i) = a {
            ship.step(&Action::E(i*waypoint.x));
            ship.step(&Action::N(i*waypoint.y));
        } else {
            waypoint.step(&a);   
        }
        // println!("{:} => {:?}", a, ship);
    }
    println!("Distance Travelled: x: {} y: {} sum: {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}

fn load() -> Vec<Action> {
    let input = tools::load(FNAME);
    input.iter().map(|x| parse_action(x)).collect_vec()
}

fn parse_action(s: &str) -> Action {
    let a = s.chars().next().unwrap();
    let b = s[1..].parse().unwrap();
    match a {
        'N' => Action::N(b),
        'S' => Action::S(b),
        'E' => Action::E(b),
        'W' => Action::W(b),
        'L' => Action::L(b),
        'R' => Action::R(b),
        'F' => Action::F(b),
        _ => Action::None(0),
    }
}