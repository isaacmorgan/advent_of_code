use std::fs;
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;

const ROCKY: i32 = 0;
const WET: i32 = 1;
const NARROW: i32 = 2;

const UNEQUIP: usize = 0;
const TORCH: usize = 1;
const GEAR: usize = 2;

const MOD: i32 = 20_183;
const MX: i32 = 16807;
const MY: i32 = 48271;

const FNAME: &str = "./input/2018-22.txt";

pub fn main() {
    let map = load();
    //println!("{:?}", map);
    //print_map_type(&map);
    println!("Small Risk: {:?}", sm_risk(&map));
    search_new(&map);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    x: usize,
    y: usize,
    item: usize,
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search_new(map: &Map) {
    let mut unvisited_nodes = BinaryHeap::new();
    let mut seen_nodes = HashSet::new();

    unvisited_nodes.push(Node { x: 0, y: 0, item: TORCH, distance: 0,});

    //println!("map: {:?}", map);
    for _i in 0..1_000_000 {
        let node = unvisited_nodes.pop().unwrap();
        //println!("Current Node: {:?}", node);

        if node.x == map.target[0] && node.y == map.target[1] && node.item == TORCH {
            println!("Current Node: {:?}", node);
            break;
        }

        let node_key = [node.x, node.y, node.item];
        if seen_nodes.contains(&node_key) {
            continue;
        }

        // Create neighbor nodes and check distance
        for dxyz in [[0, 0, 1],[0, 0, 2],[-1, 0, 0],[1, 0, 0],[0, -1, 0],[0, 1, 0]].iter() {
            if node.x as i32 + dxyz[0] < 0 || node.x as i32 + dxyz[0] >= map.x as i32 ||
                node.y as i32 + dxyz[1] < 0 || node.y as i32 + dxyz[1] >= map.y as i32 {
                continue;
            }

            let xx = (node.x as i32 + dxyz[0]) as usize;
            let yy = (node.y as i32 + dxyz[1]) as usize;
            let zz = (node.item + dxyz[2] as usize) % 3;

            if map.map_type[xx][yy] == ROCKY && zz == UNEQUIP ||
                map.map_type[xx][yy] == WET && zz == TORCH ||
                map.map_type[xx][yy] == NARROW && zz == GEAR {
                continue;
            }
            let mut neighbor = Node { x: xx, y: yy, item: zz, distance: 0, };
            neighbor.distance = get_distance(&node, &neighbor);
            if !seen_nodes.contains(&[neighbor.x, neighbor.y, neighbor.item]) {
                unvisited_nodes.push(neighbor);
            }
        }
        let node_key = [node.x, node.y, node.item];
        seen_nodes.insert(node_key);
    }
}

fn search(map: &Map) {
    let mut nodes = HashMap::new();
    let mut unvisited_nodes = HashSet::new();

    // Mark all nodes unvisited, create Unvisited Set
    // Assign every node a tentative distance value
    for x in 0..map.x {
        for y in 0..map.y {
            for item in 0..3 {
                let node = Node { x, y, item, distance: std::i32::MAX,};
                unvisited_nodes.insert([node.x, node.y, node.item]);
                nodes.insert([x, y, item], node);
            }
        }
    }

    // Assign starting node a distance of 0
    nodes.get_mut(&[0, 0, TORCH]).unwrap().distance = 0;

    // Assign starting node as current node
    //let mut current_node = nodes.get(&[0, 0, TORCH]).unwrap();
    let mut current_node = Node {x: 0, y: 0, item: TORCH, distance: 0};

    for _i in 0..10_000 {
        println!("Current Node: {:?}", current_node);
        // Check all neighbors
        for dxyz in [[0, 0, 1],[0, 0, 2],[-1, 0, 0],[1, 0, 0],[0, -1, 0],[0, 1, 0]].iter() {
            if current_node.x as i32 + dxyz[0] < 0 || current_node.x as i32 + dxyz[0] >= map.x as i32 ||
                current_node.y as i32 + dxyz[1] < 0 || current_node.y as i32 + dxyz[1] >= map.y as i32 {
                continue;
            }
            let xx = (current_node.x as i32 + dxyz[0]) as usize;
            let yy = (current_node.y as i32 + dxyz[1]) as usize;
            let zz = ((current_node.item as i32 + dxyz[2])%3) as usize;
            if map.map_type[xx][yy] == ROCKY && zz == UNEQUIP ||
                map.map_type[xx][yy] == WET && zz == TORCH ||
                map.map_type[xx][yy] == NARROW && zz == GEAR {
                continue;
            }
            let mut neighbor = nodes.get_mut(&[xx as usize, yy as usize, zz as usize]).unwrap();
            let d = get_distance(&current_node, neighbor);
            if d < neighbor.distance {
                neighbor.distance = d;
            }
        }
        if current_node.x == map.target[0] && current_node.y == map.target[1] && current_node.item == TORCH {
            println!("Current Node: {:?}", current_node);
        }
        unvisited_nodes.remove(&[current_node.x, current_node.y, current_node.item]);
        if current_node.x == map.target[0] && current_node.y == map.target[1] && current_node.item == TORCH {
            break;
        }
        let mut minval = std::i32::MAX;
        let mut current_node_ref = [0, 0, 0];
        for j in &unvisited_nodes {
            if nodes.get(j).unwrap().distance < minval {
                minval = nodes.get(j).unwrap().distance;
                current_node_ref = *j;
            }
        }
        let tmp = nodes.get(&current_node_ref).unwrap();
        current_node = Node{x: tmp.x, y: tmp.y, item: tmp.item, distance: tmp.distance};
    }
}

fn get_distance(a: &Node, b: &Node) -> i32 {
    a.distance + if a.item != b.item { 7 } else { 1 }
}

fn sm_risk(map: &Map) -> i32 {
    let mut s = 0;
    for x in 0..=map.target[0] {
        for y in 0..=map.target[1] {
            s += map.map_type[x][y];
        }
    }
    return s;
}

fn print_map_type(map: &Map) {
    for y in 0..map.y {
        for x in 0..map.x {
            print!("{}", match map.map_type[x][y] {
                ROCKY => '.',
                WET => '=',
                NARROW => '|',
                _ => '?',
            })
        }
        println!()
    }
    println!()
}

fn load() -> Map {
    let contents = fs::read_to_string(FNAME).expect("Error reading file");

    let mut depth = 0;
    let mut target = [0; 2];
    for line in contents.split("\n") {
        if line.starts_with("depth") {
            let re = regex::Regex::new(r"^depth: (\d+)").unwrap();
            for cap in re.captures_iter(line) {
                depth = cap[1].parse::<i32>().unwrap();
            }
        } else if line.starts_with("target") {
            let re = regex::Regex::new(r"^target: (\d+),(\d+)").unwrap();
            for cap in re.captures_iter(line) {
                target = [cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap()];
            }
        }
    }
    let nx = target[0]*8;
    let ny = target[1]*8;
    let map = vec![vec![0; ny]; nx];
    let mut el = vec![vec![0; ny]; nx];
    let mut gi = vec![vec![0; ny]; nx];
    let mut map_type = vec![vec![0; ny]; nx];

    for y in 0..ny {
        for x in 0..nx {
            if x == 0 && y == 0 {
                gi[x][y] = 0;
            } else if x == target[0] && y== target[1] {
                gi[x][y] = 0;
            } else if y == 0 {
                gi[x][y] = (x as i32)*MX;
            } else if x == 0 {
                gi[x][y] = (y as i32)*MY;
            } else {
                gi[x][y] = el[x-1][y] * el[x][y-1];
            }

            el[x][y] = (gi[x][y] + depth)%MOD;
            map_type[x][y] = match el[x][y]%3 {
                0 => ROCKY,
                1 => WET,
                2 => NARROW,
                _ => -1,
            }
        }
    }

    return Map {
        x: nx,
        y: ny,
        depth,
        target,
        map,
        erosion_level: el,
        geological_index: gi,
        map_type,
    }
}

#[derive(Debug)]
struct Map {
    x: usize,
    y: usize,
    depth: i32,
    target: [usize; 2],
    map: Vec<Vec<i32>>,
    erosion_level: Vec<Vec<i32>>,
    geological_index: Vec<Vec<i32>>,
    map_type: Vec<Vec<i32>>,
}
