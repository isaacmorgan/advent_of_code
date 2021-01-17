use crate::tools;
use std::collections::HashSet;
use crate::tools::Point2D;

static FNAME: &str = "./input/2020/2020-03.txt";

pub fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Map {
    trees: HashSet<Point2D<i32>>,
    size: Point2D<i32>,
}

fn part1() {
    let map = load(FNAME);
    let dx = 3;
    let dy = 1;
    let mut x = 0;
    let mut y = 0;
    let mut tree_cnt = 0;
    while y < map.size.y {
        if map.trees.contains(&Point2D{x, y}) {
            tree_cnt += 1;
        }
        x += dx;
        y += dy;
        x %= map.size.x;
    }
    println!("Tree count: {}", tree_cnt);
}

fn part2() {
    let map = load(FNAME);
    let mut tree_mult = 1;
    for (dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0;
        let mut y = 0;
        let mut tree_cnt = 0;
        while y < map.size.y {
            if map.trees.contains(&Point2D { x, y }) {
                tree_cnt += 1;
            }
            x += dx;
            y += dy;
            x %= map.size.x;
        }
        tree_mult *= tree_cnt;
    }
    println!("Tree mult: {}", tree_mult);
}

fn load(fname: &str) -> Map {
    let input = tools::load(fname);
    let mut y = 0;
    let mut x = 0;
    let mut trees = HashSet::new();
    for l in &input {
        x = 0;
        for c in l.chars() {
            if c == '#' {
                trees.insert(Point2D{x, y});
            }
            x += 1;
        }
        y += 1;
    }
    let size = Point2D {x, y};
    Map {
        trees, 
        size,
    }
}
