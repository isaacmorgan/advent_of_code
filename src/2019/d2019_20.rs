use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

const FNAME: &str = "./input/2019/2019-20.txt";

#[derive(Debug)]
struct Board {
    start: (i32, i32),
    end: (i32, i32),
    neighbors: HashMap<(i32, i32), Vec<(i32, i32)>>,
    portals: HashMap<(i32, i32), PORTAL>,
    portal_map: HashMap<(i32, i32), String>,
}

pub fn main() {
    // Scan the map for portal locations. Build a map of PortalName -> (loca, locb)
    // Scan the map for floor location. Build a map of loc -> {neighbors}
    // Record the entrance tile
    // Record the exit tile
    // For each portal location, add the portal tiles to the neighbors map
    // Do a search for shortest path
    let board = load(FNAME);
    let cost = search(&board);
    println!("Shortest Path: {}", cost);
    let cost = search_layers(&board);
    println!("Shortest Path: {}", cost);
}

fn search_layers(board: &Board) -> i32 {
    let mut explored = HashSet::new();
    let mut node = (0, 0);
    let mut cost = 0;
    let mut layer = 0;
    let mut future: Vec<((i32, i32), i32, i32)> = Vec::new();
    future.push((board.start, 0, 0));
    explored.insert((board.start.0, board.start.1, 0));
    while !future.is_empty() {
        let tmp = future.remove(0);
        node = tmp.0;
        cost = tmp.1;
        layer = tmp.2;
        //println!("node: {:?} cost: {}, layer: {}", &node, &cost, &layer);
        //println!("{:?}", future);
        if node == board.end && layer == 0 {
            return cost;
        }
        for n in board.neighbors.get(&node).unwrap() {
            let mut next_layer = layer;
            if board.portals.contains_key(n) && board.portals.contains_key(&node) {
                if let Some(a) = board.portals.get(n) {
                    match a {
                        PORTAL::IN => next_layer -= 1,
                        PORTAL::OUT => next_layer += 1,
                    }
                }
                //println!("node: {:?} cost: {}, layer: {}", &node, &cost + 1, &next_layer);
                //println!("portal {}", board.portal_map.get(&node).unwrap());
            }
            if explored.contains(&(n.0, n.1, next_layer)) {
                continue;
            }
            if next_layer < 0 {
                continue;
            }
            future.push((n.clone(), cost + 1, next_layer));
            explored.insert((n.0, n.1, next_layer));
        }
    }
    return 0;
}

fn search(board: &Board) -> i32 {
    let mut explored = HashSet::new();
    let mut node = (0, 0);
    let mut cost = 0;
    let mut future: Vec<((i32, i32), i32)> = Vec::new();
    future.push((board.start, 0));
    explored.insert(board.start);
    while !future.is_empty() {
        let tmp = future.remove(0);
        node = tmp.0;
        cost = tmp.1;
        //println!("node: {:?} cost: {}", &node, &cost);
        if node == board.end {
            return cost;
        }
        for n in board.neighbors.get(&node).unwrap() {
            if !explored.contains(n) {
                future.push((n.clone(), cost + 1));
                explored.insert(n.clone());
            }
        }
    }
    return 0;
}

#[derive(Debug)]
enum PORTAL {
    IN,
    OUT,
}

fn load(fname: &str) -> Board {
    let lines = std::fs::read_to_string(fname).unwrap();
    let lines = lines.lines();
    let mut map = Vec::new();
    for l in lines {
        println!("{:?}", &l);
        map.push(l.chars().collect_vec());
    }

    let mut paths = HashSet::new();
    let mut portals: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut portal_layers: HashMap<(i32, i32), PORTAL> = HashMap::new();
    let mut portal_map = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                'A'..='Z' => {
                    for dxy in &[(0, 1), (1, 0)] {
                        let x1 = x as i32 + dxy.0;
                        let y1 = y as i32 + dxy.1;
                        if x1 < map[0].len() as i32 && y1 < map.len() as i32 && map[y1 as usize][x1 as usize] >= 'A' && map[y1 as usize][x1 as usize] <= 'Z' {
                            let portal_name = String::from_iter(vec![map[y][x], map[y1 as usize][x1 as usize]]);
                            // Get tile location
                            for dxy2 in &[(2 * dxy.0, 2 * dxy.1), (-dxy.0, -dxy.1)] {
                                let x2 = x as i32 + dxy2.0;
                                let y2 = y as i32 + dxy2.1;
                                if x2 >= 0 && x2 < map[0].len() as i32 && y2 >= 0 && y2 < map.len() as i32 && map[y2 as usize][x2 as usize] == '.' {
                                    if portals.contains_key(&portal_name) {
                                        let mut v = portals.remove(&portal_name).unwrap();
                                        v.push((x2, y2));
                                        portals.insert(portal_name.clone(), v);
                                    } else {
                                        portals.insert(portal_name.clone(), vec![(x2, y2)]);
                                    }
                                    portal_map.insert((x2, y2), portal_name.clone());
                                    // Add to portal layers map
                                    if x2 > 2 && y2 > 2 && x2 < map[0].len() as i32 - 3 && y2 < map.len() as i32 - 3 {
                                        portal_layers.insert((x2, y2), PORTAL::IN);
                                    } else {
                                        portal_layers.insert((x2, y2), PORTAL::OUT);
                                    }
                                }
                            }
                        }
                    }
                }
                '.' => { paths.insert((x as i32, y as i32)); }
                _ => (),
            }
        }
    }

    let mut neighbors = HashMap::new();
    for p in &paths {
        for dxy in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let np = (p.0 + dxy.0, p.1 + dxy.1);
            if paths.contains(&np) {
                let v = neighbors.entry(*p).or_insert(Vec::new());
                v.push(np);
            }
        }
    }

    for (_, v) in &portals {
        for p in v {
            for q in v {
                if p == q {
                    continue;
                }
                let w = neighbors.entry(*p).or_insert(Vec::new());
                if !w.contains(q) {
                    w.push(*q);
                }
            }
        }
    }

    let start = *portals.get("AA").unwrap().first().unwrap();
    let end = *portals.get("ZZ").unwrap().first().unwrap();

    return Board { start, end, neighbors, portals: portal_layers, portal_map };
    //dbg!(paths, portals, neighbors);
}