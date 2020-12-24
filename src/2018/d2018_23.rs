use std::cmp::{Ordering, max, min};
use std::collections::{BinaryHeap, HashSet};

const FNAME: &str = "./input/2018-23.txt";

pub fn main() {
    let mut bots = load();
    bots.sort_by(|a, b| b.r.cmp(&a.r));
    let largest = bots.get(0).unwrap().clone();
    println!("Largest: {:?}", largest);
    println!("N: {}", bots.len());
    let bots_in_range = bots.iter().filter(|b| in_range(&largest, b));
    println!("Bots in range of largest: {}", bots_in_range.count());

    bots.sort_by(|a, b| b.neighbor_count.cmp(&a.neighbor_count));
    for b in &bots {
        println!("{} - {} - {}", b.id, b.r, b.neighbor_count);
    }

    let mut friends: Vec<Bot> = Vec::new();

    'outer:
    for b in &bots {
        for f in &friends {
            if !b.neighbors.contains(&f.id) {
                continue 'outer;
            }
        }
        friends.push(b.clone());
    }

    println!("Friends: {}", friends.len());

    let mut boxes = Vec::new();
    for f in &friends {
        boxes.push(bot_to_box(f));
    }
    let mut sumbox: Box = boxes.remove(0);
    for b in &boxes {
        println!("{:?}", sumbox);
        println!("{:?}", b);
        sumbox = add_box_to_box(b, &sumbox).unwrap();
    }

    println!("Num bots: {}", sumbox.ids.len());

    println!("{:?}", sumbox);

    let mut mval = 0;
    for id in sumbox.ids {
        for b in &bots {
            if b.id == id {
                println!("{} - {} : {}", b.pos[0].abs() + b.pos[1].abs() + b.pos[2].abs(), b.r, b.pos[0].abs() + b.pos[1].abs() + b.pos[2].abs() - b.r);
                mval = max(b.pos[0].abs() + b.pos[1].abs() + b.pos[2].abs() - b.r, mval);
                println!("this sucks: {}", mval);
                continue;
            }
        }
    }

    println!("this sucks: {}", mval);
/*
    let mut cnt = 0;
    for x in sumbox.x[0]..=sumbox.x[1] {
        for y in sumbox.y[0]..=sumbox.y[1] {
            cnt += 1;
        }
    }
    println!("{}", cnt);

    let mut mincnt = 9999;
    for mut b in &bots {
        let mut cnt = 0;
        for c in &bots {
            if in_mutual_range(b, c) {
                cnt += 1;
            }
        }
        //println!("{} - {}", b.id, cnt);
        //b.neighbor_count = cnt;
        mincnt = min(cnt, mincnt);
    }
    println!("{}", mincnt);
    */

    /*
    // Create set of one box for each bot
    let mut b1 = Vec::new();
    for b in &bots {
        b1.push(bot_to_box(b));
    }

    let mut boxes = Vec::new();
    for b in &bots {
        boxes.push(bot_to_box(b));
    }

    for i in 0..boxes.len() {
        for j in (i+1)..boxes.len() {
            match add_box_to_box(&boxes[i], &boxes[j]) {
                None => {
                    let id1 = boxes[i].ids.clone();
                    let id2 = boxes[j].ids.clone();
                    boxes[i].exclude.extend(id2.iter());
                    boxes[j].exclude.extend(id1.iter());
                },
                Some(_) => (),
            }
        }
    }

    // Combine boxes until all options are created
    let mut combo_boxes = Vec::new();
    loop {
        println!("ok");
        combo_boxes = Vec::new();
        for i in 0..boxes.len() {
            for j in (i+1)..boxes.len() {
                match add_box_to_box(&boxes[i], &boxes[j]) {
                    None => (),
                    Some(box3) => combo_boxes.push(box3),
                }
            }
        }
        if combo_boxes.is_empty() {
            break
        } else {
            println!("This many combos: {}", combo_boxes.len());
            boxes = combo_boxes;
        }
    }

    for b in &combo_boxes {
        println!("Box: {:?}", b);
    }
    println!("Box count: {}", &combo_boxes.len());
*/
}

fn search() {
    // For each bot, find links to other bots
    // Start with bot 1
    // Iterate over all direct links
    // Follow secondary links not in current chain
    // Check if secondary link is compatible

}

#[derive(Debug, Clone)]
struct Box {
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
    ids: HashSet<i64>,
    exclude: HashSet<i64>,
}

fn add_box_to_box(box1: &Box, box2: &Box) -> Option<Box> {
    for id in &box1.ids {
        if box2.ids.contains(id) || box2.exclude.contains(id) {
            println!("a");
            return None
        }
    }

    for id in &box2.ids {
        if box1.exclude.contains(id) {
            println!("b");
            return None
        }
    }
    let c1 = ((box1.x[0] >= box2.x[0] && box1.x[0] <= box2.x[1]) || (box1.x[1] >= box2.x[0] && box1.x[1] <= box2.x[1]) || (box1.x[0] <= box2.x[0] && box1.x[1] >= box2.x[1]));
    let c2 = ((box1.y[0] >= box2.y[0] && box1.y[0] <= box2.y[1]) || (box1.y[1] >= box2.y[0] && box1.y[1] <= box2.y[1]) || (box1.y[0] <= box2.y[0] && box1.y[1] >= box2.y[1]));
    let c3 =  ((box1.z[0] >= box2.z[0] && box1.z[0] <= box2.z[1]) || (box1.z[1] >= box2.z[0] && box1.z[1] <= box2.z[1]) || (box1.z[0] <= box2.z[0] && box1.z[1] >= box2.z[1]));
    //println!("{} {} {}", c1, c2, c3);
    if c1 && c2 && c3 {
        let x = [max(box1.x[0], box2.x[0]), min(box1.x[1], box2.x[1])];
        let y = [max(box1.y[0], box2.y[0]), min(box1.y[1], box2.y[1])];
        let z = [max(box1.z[0], box2.z[0]), min(box1.z[1], box2.z[1])];
        let mut ids = box2.ids.clone();
        let mut exclude = box2.exclude.clone();
        for id in &box1.ids {
            ids.insert(*id);
        }
        for ex in &box1.exclude {
            exclude.insert(*ex);
        }
        return Some(Box { x, y, z, ids, exclude })
    }

    println!("c");
    return None;
}

fn bot_to_box(bot: &Bot) -> Box {
    Box {
        x: [bot.pos[0] - bot.r, bot.pos[0] + bot.r],
        y: [bot.pos[1] - bot.r, bot.pos[1] + bot.r],
        z: [bot.pos[2] - bot.r, bot.pos[2] + bot.r],
        ids: vec![bot.id].into_iter().collect(),
        exclude: HashSet::new(),
    }
}

fn box_it_up() {
    // For each nanobot, generate bounding box -> save as weight 1 bounding box and list of nanobot ids
    // Compare each bounding box with each other, generating a new set of weight 2 bounding boxes
    // Compare each weight n bounding box with each weight 1 bounding box, generating weight n + 1 bounding boxes
    // Stop when no longer any new boxes
}

fn in_mutual_range(a: &Bot, b: &Bot) -> bool {
    (a.r + b.r) >= (b.pos[0] - a.pos[0]).abs() + (b.pos[1] - a.pos[1]).abs() + (b.pos[2] - a.pos[2]).abs()
}

fn in_range(a: &Bot, b: &Bot) -> bool {
    // wtf uses Manhattan distance, just terrible
    a.r >= (b.pos[0] - a.pos[0]).abs() + (b.pos[1] - a.pos[1]).abs() + (b.pos[2] - a.pos[2]).abs()
}

fn load() -> Vec<Bot> {
    let mut bots = Vec::new();

    let mut contents = std::fs::read_to_string(FNAME).expect("Error reading file");
    let re = regex::Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut cnt = 0;
    for line in contents.split("\n") {
        for cap in re.captures_iter(line) {
            cnt += 1;
            let id = cnt;
            let pos = [cap[1].parse::<i64>().unwrap(), cap[2].parse::<i64>().unwrap(), cap[3].parse::<i64>().unwrap()];
            let r = cap[4].parse::<i64>().unwrap();
            bots.push(Bot {id, pos, r, neighbors: HashSet::new(), neighbor_count: 0})
        }
    }

    for i in 0..bots.len() {
        let mut neighbor_count = 0;
        let mut neighbors = HashSet::new();
        for j in 0..bots.len() {
            if in_mutual_range(&bots[i], &bots[j]) {
                neighbor_count += 1;
                neighbors.insert(*(&bots[j].id));
            }
        }
        bots[i].neighbor_count = neighbor_count;
        bots[i].neighbors = neighbors;
    }

    return bots;
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Bot {
    id: i64,
    pos: [i64; 3],
    r: i64,
    neighbors: HashSet<i64>,
    neighbor_count: i64,
}

impl Ord for Bot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r.cmp(&other.r)
    }
}

impl PartialOrd for Bot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}