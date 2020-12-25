use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

mod intcode;

const FNAME: &str = "./input/2019/2019-15.txt";

pub fn main() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program = program;
    intcode::reboot_computer(&mut comp);

    let pos = (0, 0);
    let mut map = HashMap::new();

    explore_map(&pos, &mut comp, &mut map);
    print_map(&map);
    intcode::reboot_computer(&mut comp);
    let mut goals = HashSet::new();
    let mut start = (0,0);
    for (d, m) in &map {
        if m == &'O' {
            goals.insert(d.clone());
            start = d.clone();
            break;
        }
    }
    let inst = uniform_cost_search(&(0, 0), &goals, &map);
    println!("Distance to Site: {}", inst.0.len());

    goals.clear();
    let (_, maxv) = uniform_cost_search(&start, &goals, &map);
    println!("Max oxygen time: {}", maxv.0);
}

fn explore_map(start: &(i32, i32), mut comp: &mut intcode::Computer, map: &mut HashMap<(i32, i32), char>) {
    let mut pos = start.clone();
    let mut unknown = HashSet::new();

    loop {
        // Keep list of unexplored neighbors
        for i in 0..4 {
            let neighbor = match i {
                0 => (pos.0, pos.1 + 1),
                1 => (pos.0, pos.1 - 1),
                2 => (pos.0 - 1, pos.1),
                3 => (pos.0 + 1, pos.1),
                _ => (pos.0, pos.1),
            };
            if !map.contains_key(&neighbor) {
                unknown.insert(neighbor);
            }
        }

        // Get instructions to move to nearest neighbor
        let (inst, dest) = uniform_cost_search(&pos, &unknown, &map);
        let n = inst.len();
        //dbg!(&pos, &unknown);
        //dbg!(n);
        // Execute instructions and record map
        if n == 0 {
            break;
        }
        let last_inst = inst.last().unwrap().clone();
        for t in inst {
            comp.input.push(t as i64);
        }
        while comp.output.len() < n {
            intcode::step(&mut comp);
        }
        /*loop {
        if comp.output.len() == 0 {
            intcode::step(&mut comp);
        } else {
            break;
        }
    }*/

        let dpos = match last_inst {
            1 => (0, 1),
            2 => (0, -1),
            3 => (-1, 0),
            4 => (1, 0),
            _ => (0, 0),
        };

        let status = comp.output.last().unwrap();
        match status {
            0 => {
                pos = (dest.0.clone() - dpos.0, dest.1.clone() - dpos.1);
                map.insert(dest, '#');
            },
            1 => {
                pos = dest.clone();
                map.insert(dest, '.');
            },
            2 => {
                pos = dest.clone();
                map.insert(dest, 'O');
            },
            _ => (),
        }
        comp.output.clear();

        // Update list of unexplored neighbors
        for (m,_) in &*map {
            if unknown.contains(&m) {
                unknown.remove(&m);
            }
        }

        if unknown.is_empty() {
            break;
        }
    }
}

fn uniform_cost_search(start: &(i32, i32), goals: &HashSet<(i32, i32)>, map: &HashMap<(i32, i32), char>)
    -> (Vec<i32>, (i32, i32)) {
    let mut node = start.clone();
    let mut cost = 0;
    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    let mut min_cost = HashMap::new();
    let mut instructions = Vec::new();
    frontier.push(node.clone());
    min_cost.insert(node.clone(), cost.clone());
    loop {
        //println!("Frontier: {:?}", &frontier);
        if frontier.is_empty() {
            return (instructions, (cost.clone(), cost.clone()));
        }
        node = frontier.remove(0);
        //dbg!(&frontier, &node, &min_cost);
        cost = min_cost.get(&node).unwrap().clone();
        let cost2 = cost.clone();
        if goals.contains(&node) {
            let dest = node.clone();
            //println!("Destination: {:?}", node);
            while cost > 0 {
                cost -= 1;
                for i in 0..4 {
                    let guess_prev = match i {
                        0 => (node.0, node.1 - 1),
                        1 => (node.0, node.1 + 1),
                        2 => (node.0 + 1, node.1),
                        3 => (node.0 - 1, node.1),
                        _ => (node.0, node.1),
                    };
                    if min_cost.contains_key(&guess_prev) && min_cost.get(&guess_prev).unwrap() == &cost {
                        node = guess_prev;
                        instructions.insert(0, i + 1);
                        break;
                    }
                }
            }
            return (instructions, dest);
        }
        explored.insert(node);
        for i in 0..4 {
            let next = match i {
                0 => (node.0, node.1 + 1),
                1 => (node.0, node.1 - 1),
                2 => (node.0 - 1, node.1),
                3 => (node.0 + 1, node.1),
                _ => (0, 0),
            };
            if (map.contains_key(&next)
              && !frontier.contains(&next)
              && !explored.contains(&next)
              && map.get(&next).unwrap() != &'#') || goals.contains(&next) {
                frontier.push(next);
                min_cost.insert(next.clone(), cost2 + 1);
                //println!("frontier: {:?} mincost: {:?}", frontier, min_cost);
            }
        }
    }
}

fn print_map(map: &HashMap<(i32, i32), char>) {
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for (p,_) in map {
        xmin = min(xmin, p.0);
        xmax = max(xmax, p.0);
        ymin = min(ymin, p.1);
        ymax = max(ymax, p.1);
    }

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            print!("{}", map.get(&(x,y)).unwrap_or(&' '));
        }
        println!();
    }
}
