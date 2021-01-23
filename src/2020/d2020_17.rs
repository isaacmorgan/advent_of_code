use crate::tools;
use itertools::enumerate;
use std::collections::{HashMap, HashSet};

static FNAME: &str = "./input/2020/2020-17.txt";

#[derive(Debug, Eq, Hash, PartialOrd, PartialEq, Copy, Clone)]
struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let mut map = load();
    println!("{:?}", map);
    let n = 6;
    for i in 0..n {
        step(&mut map);
    }
    println!("Active cubes after {} iterations: {}", n, map.len());
}

fn part2() {
    let mut map = load4();
    println!("{:?}", map);
    let n = 6;
    for i in 0..n {
        step4(&mut map);
    }
    println!("Active cubes after {} iterations: {}", n, map.len());
}

fn step(map: &mut HashSet<(i32, i32, i32)>) {
    let b = bounds(map);
    let mut add = Vec::new();
    let mut rem = Vec::new();
    for x in b.0.0..=b.0.1 {
        for y in b.1.0..=b.1.1 {
            for z in b.2.0..=b.2.1 {
                let cnt = count_neighbors((x, y, z), &map);
                let s = map.contains(&(x, y, z));
                if s && cnt != 2 && cnt != 3 {
                    rem.push((x, y, z));
                } else if !s && cnt == 3 {
                    add.push((x, y, z));
                }
            }
        }
    }
    for r in &rem {
        map.remove(r);
    }
    for a in add {
        map.insert(a);
    }
}

fn step4(map: &mut HashSet<Point4D>) {
    let b = bounds4(map);
    let mut add = Vec::new();
    let mut rem = Vec::new();
    for x in b.0.0..=b.0.1 {
        for y in b.1.0..=b.1.1 {
            for z in b.2.0..=b.2.1 {
                for w in b.3.0..=b.3.1 {
                    let p4 = Point4D{ x, y, z, w};
                    let cnt = count_neighbors4(p4.clone(), &map);
                    let s = map.contains(&p4);
                    if s && cnt != 2 && cnt != 3 {
                        rem.push(p4);
                    } else if !s && cnt == 3 {
                        add.push(p4);
                    }
                }
            }
        }
    }
    for r in &rem {
        map.remove(r);
    }
    for a in add {
        map.insert(a);
    }
}

fn count_neighbors(i: (i32, i32, i32), map: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut cnt = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                if map.contains(&(i.0 + dx, i.1 + dy, i.2 + dz)) {
                    cnt += 1;
                }
                if cnt > 3 {
                    return cnt;
                }
            }
        }
    }
    return cnt;
}

fn count_neighbors4(i: Point4D, map: &HashSet<Point4D>) -> i32 {
    let mut cnt = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }
                    if map.contains(&Point4D {
                        x: i.x + dx, 
                        y: i.y + dy, 
                        z: i.z + dz, 
                        w: i.w + dw,
                    }) {
                        cnt += 1;
                    }
                    if cnt > 3 {
                        return cnt;
                    }
                }
            }
        }
    }
    return cnt;
}

fn bounds(map: &HashSet<(i32, i32, i32)>) -> ((i32, i32), (i32, i32), (i32, i32)) {
    let mut x = (0, 0);
    let mut y = (0, 0);
    let mut z = (0, 0);
    for m in map {
        if m.0 < x.0 {
            x.0 = m.0;
        }
        if m.0 > x.1 {
            x.1 = m.0;
        }
        if m.1 < y.0 {
            y.0 = m.1;
        }
        if m.1 > y.1 {
            y.1 = m.1;
        }
        if m.2 < z.0 {
            z.0 = m.2;
        }
        if m.2 > z.1 {
            z.1 = m.2;
        }
    }
    x.0 = x.0 - 1;
    x.1 = x.1 + 1;
    y.0 = y.0 - 1;
    y.1 = y.1 + 1;
    z.0 = z.0 - 1;
    z.1 = z.1 + 1;
    return (x, y, z);
}

fn bounds4(map: &HashSet<Point4D>) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    let mut x = (0, 0);
    let mut y = (0, 0);
    let mut z = (0, 0);
    let mut w = (0, 0);
    for m in map {
        if m.x < x.0 {
            x.0 = m.x;
        }
        if m.x > x.1 {
            x.1 = m.x;
        }
        if m.y < y.0 {
            y.0 = m.y;
        }
        if m.y > y.1 {
            y.1 = m.y;
        }
        if m.z < z.0 {
            z.0 = m.z;
        }
        if m.z > z.1 {
            z.1 = m.z;
        }
        if m.w < w.0 {
            w.0 = m.w;
        }
        if m.w > w.1 {
            w.1 = m.w;
        }
    }
    x.0 = x.0 - 1;
    x.1 = x.1 + 1;
    y.0 = y.0 - 1;
    y.1 = y.1 + 1;
    z.0 = z.0 - 1;
    z.1 = z.1 + 1;
    w.0 = w.0 - 1;
    w.1 = w.1 + 1;
    return (x, y, z, w);
}

fn load() -> HashSet<(i32, i32, i32)> {
    let input = tools::load(FNAME);
    let mut map = HashSet::new();
    let z = 0;
    for (y, line) in enumerate(input) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                map.insert((x as i32, y as i32, z));
            }
        }
    }
    return map;
}

fn load4() -> HashSet<Point4D> {
    let input = tools::load(FNAME);
    let mut map = HashSet::new();
    let z = 0;
    let w = 0;
    for (y, line) in enumerate(input) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                map.insert(Point4D {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
    }
    return map;
}