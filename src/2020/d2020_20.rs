use crate::tools;
use itertools::{enumerate, Itertools, multizip};
use std::collections::{HashMap, HashSet};
use std::cmp::min;

static FNAME: &str = "./input/2020/2020-20.txt";

// This is not the worst code I've ever written.
// The worst code I've ever written was the first version of this. 
// Be thankful I didn't commit that.

#[derive(Debug, Clone)]
struct Tile {
    id: i32,
    data: Vec<char>,
    x: usize,
    y: usize,
    neighbors: Vec<i32>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut map = String::new();
        for y in 0..self.y {
            map.extend(self.data[(y * self.x)..((y + 1) * self.x)].iter());
            map.extend("\n".chars());
        }
        write!(f, "\nid: {}\nx: {} y: {} neighbors ({}): {:?}\n{}\n", self.id, self.x, self.y, self.neighbors.len(), self.neighbors, map)
    }
}

impl Tile {
    fn get(&self, x: &usize, y: &usize) -> char {
        self.data[y * self.x + x]
    }

    fn flip_lr(&mut self) {
        let mut old_data = self.data.clone();
        for x in 0..self.x {
            for y in 0..self.y {
                self.data[y * self.x + x] = old_data[y * self.x + (self.x - 1 - x)];
            }
        }
    }

    fn flip_ud(&mut self) {
        let mut old_data = self.data.clone();
        for x in 0..self.x {
            for y in 0..self.y {
                self.data[y * self.x + x] = old_data[(self.y - 1 - y) * self.x + x];
            }
        }
    }

    fn rotate_ccw(&mut self) {
        let mut old_data = self.data.clone();
        for x in 0..self.x {
            for y in 0..self.y {
                self.data[y * self.x + x] = old_data[x * self.x + (self.y - 1 - y)];
            }
        }
    }

    fn edges(&self) -> Vec<Vec<char>> {
        let mut edges = Vec::new();
        for side in 0..4 {
            edges.push(self.get_edge(side, false));
        }
        return edges;
    }

    fn get_edge(&self, id: usize, rev: bool) -> Vec<char> {
        let mut edge = Vec::new();
        match id {
            0 => { // Right
                let x = self.x - 1;
                for y in 0..self.y {
                    edge.push(self.get(&x, &y));
                }
            }
            1 => { // Bottom
                let y = self.y - 1;
                for x in (0..self.x).rev() {
                    edge.push(self.get(&x, &y));
                }
            }
            2 => { // Left
                let x = 0;
                for y in (0..self.y).rev() {
                    edge.push(self.get(&x, &y));
                }
            }
            3 => { // Top
                let y = 0;
                for x in 0..self.x {
                    edge.push(self.get(&x, &y));
                }
            }
            _ => (),
        }
        if rev {
            edge.reverse();
        }
        return edge;
    }
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let mut tiles = load();
    println!("tiles: {:?}", tiles);
    calc_neighbors(&mut tiles);
    let corners = tiles.iter().filter(|x| x.neighbors.len() == 2).map(|x| x.id).collect_vec();
    println!("corners: {:?}", corners);
    println!("corners mult: {}", corners.iter().map(|x| *x as i64).product::<i64>());
}

fn part2() {
    let mut tiles = load();
    println!("tiles: {:?}", tiles);
    calc_neighbors(&mut tiles);
    let corners = tiles.iter().filter(|x| x.neighbors.len() == 2).map(|x| x.id).collect_vec();
    let tile_map = calc_tile_map(&tiles);
    let mut tile_set: HashSet<i32> = HashSet::new();
    tile_set.extend(tile_map.keys());
    let mut tile_order = Vec::new();
    align_corner_tl(&corners[0], &mut tiles, &tile_map);
    tile_set.remove(&corners[0]);
    tile_order.push(corners[0]);
    // println!("{}", &tiles[*tile_map.get(&tile_order.first().unwrap()).unwrap()]);
    for i in 0..tile_set.len() {
        place_next_tile(&mut tiles, &mut tile_set, &mut tile_order, &tile_map);
        // println!("{}", &tiles[*tile_map.get(&tile_order.last().unwrap()).unwrap()]);
    }
    // for id in &tile_order {
    //     println!("{}", tiles[*tile_map.get(id).unwrap()]);
    // }
    // println!("{:?}", tile_order);
    let map = calc_map(&tiles, &tile_order, &tile_map);
    print_map(&map);
    let serpent_str = "                  # #    ##    ##    ### #  #  #  #  #  #   ";
    let mut serpent = Vec::new();
    for x in 0..20 {
        for y in 0..3 {
            if &serpent_str[(y*20 + x)..=(y*20 + x)] == "#" {
                serpent.push((x, y));
            }
        }
    }
    println!("{:?}", serpent);
    let s_cnt = count_serpents(&map, &serpent);
    println!("Serpent Count: {}", s_cnt);
    println!("Roughness: {}", map.len() as u32 - s_cnt*(serpent_str.chars().filter(|x| *x == '#').count() as u32));
}

fn print_map(map: &HashSet<(usize, usize)>) {
    let xmax = map.iter().map(|(x, y)| *x).max().unwrap();
    let ymax = map.iter().map(|(x, y)| *y).max().unwrap();
    for x in 0..=xmax {
        for y in 0..=ymax {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn count_serpents(map: &HashSet<(usize, usize)>, serpent: &Vec<(usize, usize)>) -> u32 {
    let xmax = map.iter().map(|(x, _)| *x).max().unwrap();
    let ymax = map.iter().map(|(_, y)| *y).max().unwrap();
    let sa = serpent.clone();
    let sb = rotate_serpent(&sa);
    let sc = rotate_serpent(&sb);
    let sd = rotate_serpent(&sc);
    let se = flip_serpent(&sa);
    let sf = flip_serpent(&sb);
    let sg = flip_serpent(&sc);
    let sh = flip_serpent(&sd);
    let mut max_cnt = 0;
    for s in &[sa, sb, sc, sd, se, sf, sg, sh] {
        // println!("{:?}", s);
        let mut cnt = 0;
        let wx = s.iter().map(|(x, _)| *x).max().unwrap();
        let wy = s.iter().map(|(_, y)| *y).max().unwrap();
        for x in 0..=xmax-wx {
            'outer:
            for y in 0..=ymax-wy {
                for (sx, sy) in s {
                    if map.contains(&(x + sx, y + sy)) {
                        continue;
                    }
                    continue 'outer;
                }
                cnt += 1;
            }
        }
        // println!("cnt: {}", cnt);
        if cnt > max_cnt {
            max_cnt = cnt;
        }
    }
    return max_cnt;
}

fn rotate_serpent(serpent: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_serpent = Vec::new();
    let dx = serpent.iter().map(|(x, _)| *x).max().unwrap();
    let dy = serpent.iter().map(|(_, y)| *y).max().unwrap();
    for (x, y) in serpent {
        let xx = dy - y;
        let yy = *x;
        new_serpent.push((xx, yy));
    }
    return new_serpent;
}

fn flip_serpent(serpent: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_serpent = Vec::new();
    let dx = serpent.iter().map(|(x, _)| *x).max().unwrap();
    let dy = serpent.iter().map(|(_, y)| *y).max().unwrap();
    for (x, y) in serpent {
        let xx = dx - x;
        let yy = *y;
        new_serpent.push((xx, yy));
    }
    return new_serpent;
}

fn calc_map(tiles: &Vec<Tile>, tile_order: &Vec<i32>, tile_map: &HashMap<i32, usize>) -> HashSet<(usize, usize)> {
    let mut map = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    for (i, id) in enumerate(tile_order) {
        if tiles[*tile_map.get(id).unwrap()].neighbors.len() as i32 == 2 && i > 0 {
            w = i + 1;
            break;
        }
    }
    for (i, id) in enumerate(tile_order) {
        let t = &tiles[*tile_map.get(id).unwrap()];
        // dbg!(" ", x, y, i, w, i%w);
        if i%w == 0 && i > 0 {
            y += 1;
            x = 0;
        } else if i > 0 {
            x += 1;
        }
        for xx in 1..t.x - 1 {
            for yy in 1..t.y - 1 {
                if t.get(&xx, &yy) == '#' {
                    map.insert((x * (t.x - 2) + xx - 1, y * (t.y - 2) + yy - 1));
                }
            }
        }
    }
    return map;
}

fn place_next_tile(tiles: &mut Vec<Tile>, tile_set: &mut HashSet<i32>, tile_order: &mut Vec<i32>, tile_map: &HashMap<i32, usize>) {
    let d = (tiles.len() as f32).sqrt().round() as usize;
    let mut p;
    let ep;
    let mode = if tile_order.len() % d == 0 {
        p = &tiles[*tile_map.get(tile_order.get(tile_order.len() - d).unwrap()).unwrap()];
        ep = p.get_edge(1, false);
        1
    } else {
        p = &tiles[*tile_map.get(tile_order.last().unwrap()).unwrap()];
        ep = p.get_edge(0, false);
        0
    };
    let mut n_rot = 0;
    let mut n_flip = false;
    let mut n_val = 0;
    // println!();
    'outer:
    for n_id in &p.neighbors {
        if !tile_set.contains(&n_id) {
            continue;
        }
        // dbg!(n_id, tile_order.len());
        let q = &tiles[*tile_map.get(n_id).unwrap()];
        let side = neighbor_which_side(p, q);
        // println!("{}", p);
        // println!("{}", q);
        // dbg!(mode, side);
        if mode == 0 && side == 0 {
            // println!("right");
            let mut edges = q.edges();
            for (i, mut e) in enumerate(edges) {
                if ep == e {
                    n_rot = ((i + 2) % 4);
                    n_flip = true;
                    n_val = *n_id;
                    break 'outer;
                }
                e.reverse();
                if ep == e {
                    n_rot = ((i + 2) % 4);
                    n_flip = false;
                    n_val = *n_id;
                    break 'outer;
                }
            }
        } else if mode == 1 && side == 1 {
            // println!("down");
            let mut edges = q.edges();
            for (i, mut e) in enumerate(edges) {
                if ep == e {
                    n_rot = ((i + 1) % 4);
                    n_flip = true;
                    n_val = *n_id;
                    break 'outer;
                }
                e.reverse();
                if ep == e {
                    n_rot = ((i + 1) % 4);
                    n_flip = false;
                    n_val = *n_id;
                    break 'outer;
                }
            }
        }
    }
    // dbg!(n_rot, n_flip, n_val);

    let mut q = &mut tiles[*tile_map.get(&n_val).unwrap()];
    for _ in 0..n_rot {
        q.rotate_ccw();
    }
    if n_flip {
        if mode == 0 {
            q.flip_ud();
        } else if mode == 1 {
            q.flip_lr();
        }
    }

    tile_set.remove(&n_val);
    tile_order.push(n_val);
}

fn calc_tile_map(tiles: &Vec<Tile>) -> HashMap<i32, usize> {
    let mut tile_map = HashMap::new();
    for (i, t) in enumerate(tiles) {
        tile_map.insert(t.id, i);
    }
    return tile_map;
}

fn align_corner_tl(tile_id: &i32, tiles: &mut Vec<Tile>, tile_map: &HashMap<i32, usize>) {
    let mut tile = &tiles[*tile_map.get(tile_id).unwrap()];
    let n_a = &tiles[*tile_map.get(&tile.neighbors[0]).unwrap()];
    let n_b = &tiles[*tile_map.get(&tile.neighbors[1]).unwrap()];
    let size_a = neighbor_which_side(tile, n_a);
    let size_b = neighbor_which_side(tile, n_b);

    let mut tile = &mut tiles[*tile_map.get(tile_id).unwrap()];
    if (size_a == 0 && size_b == 3) || (size_a == 3 && size_b == 0) {
        for _ in 0..3 {
            tile.rotate_ccw();
        }
    } else {
        for _ in 0..min(size_a, size_b) {
            tile.rotate_ccw();
        }
    }
}

fn neighbor_which_side(tile: &Tile, neighbor: &Tile) -> usize {
    let edges_a = tile.edges();
    let mut edges_b = neighbor.edges();
    // println!("{:?}", edges_a);
    // println!("{:?}", edges_b);
    for (i, ea) in enumerate(&edges_a) {
        for mut eb in edges_b.iter_mut() {
            if ea == eb {
                return i;
            }
            eb.reverse();
            if ea == eb {
                return i;
            }
        }
    }
    return 4;
}

fn calc_neighbors(tiles: &mut Vec<Tile>) {
    let tlen = tiles.len();
    for i in 0..tlen {
        for j in i + 1..tlen {
            let (a, b) = tiles.split_at_mut(j);
            let mut t0 = a.get_mut(i).unwrap();
            let mut t1 = b.first_mut().unwrap();
            if is_neighbor(&t0, &t1) {
                t0.neighbors.push(t1.id);
                t1.neighbors.push(t0.id);
            }
        }
    }
}

fn is_neighbor(a: &Tile, b: &Tile) -> bool {
    // compare all sides of a with all sides of b
    let edges_a = a.edges();
    let edges_b = b.edges();
    for ea in &edges_a {
        for eb in &edges_b {
            if ea == eb {
                return true;
            }
            let mut ebr = eb.clone();
            ebr.reverse();
            if ea == &ebr {
                return true;
            }
        }
    }
    return false;
}

fn load() -> Vec<Tile> {
    let input = tools::load(FNAME);
    let mut tiles = Vec::new();
    let mut mode = 0;

    // Get tile size
    let x = input[1].len();
    let y = input.iter().position(|x| x.is_empty()).unwrap() - 1;

    let mut tile_template = Tile {
        id: 0,
        data: vec![],
        x,
        y,
        neighbors: vec![],
    };

    for line in input {
        if line.is_empty() {
            tiles.push(tile_template.clone());
            mode = 0;
            continue;
        }
        if mode == 0 {
            tile_template.id = line.trim_start_matches("Tile ").trim_end_matches(":").parse().unwrap();
            tile_template.data.clear();
            mode += 1;
        } else {
            tile_template.data.extend(line.chars());
        }
    }

    return tiles;
}
