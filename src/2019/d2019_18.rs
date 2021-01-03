use std::collections::{HashMap, HashSet};
use itertools::{Itertools, enumerate};
use std::iter::FromIterator;

const FNAME: &str = "./input/2019/2019-18.txt";

#[derive(Debug, Clone)]
struct State {
    pos: Vec<char>,
    keys: HashSet<char>,
    cost: i32,
}

#[derive(Debug, Clone)]
struct Path {
    a: char,
    b: char,
    cost: i32,
    keys: HashSet<char>,
}

/*
impl PartailEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
        && self.keys == other.keys
    }
}*/

pub fn main() {
    let mut map = load(FNAME);
    println!("{:?}", map);
    let pos = get_pos(&map);
    println!("pos: {:?}", pos);
    let goals = get_goals(&map);
    println!("goals: {:?}", goals);
    print_map(&map);

    //part_1(&map);
    part_2(&map);
}

fn part_2(map: &HashMap<(i32, i32), char>) {
    let mut map = map.clone();
    let c = self::get_location(&map, '@').unwrap();
    map.insert((c.0 - 1, c.1 - 1), '0');
    map.insert((c.0 - 1, c.1 + 1), '1');
    map.insert((c.0 + 1, c.1 - 1), '2');
    map.insert((c.0 + 1, c.1 + 1), '3');
    map.insert((c.0 + 1, c.1), '#');
    map.insert((c.0 - 1, c.1), '#');
    map.insert((c.0, c.1 + 1), '#');
    map.insert((c.0, c.1 - 1), '#');
    map.insert(c, '#');
    print_map(&map);
    let paths = get_all_paths(&map);
    println!("{:?}", paths);
    let goals = get_goals(&map);
    println!("Goal length: {}", goals.len());
    let final_state = cost_search_paths(&paths, goals.len() as i32 + 4, vec!['0','1','2','3']);
    println!("Final state: {:?}", final_state);
}

fn part_1(map: &HashMap<(i32, i32), char>) {
    let paths = get_all_paths(map);
    let goals = get_goals(map);
    println!("Goal length: {}", goals.len());
    let final_state = cost_search_paths(&paths, goals.len() as i32 + 1, vec!['@']);
    println!("Final state: {:?}", final_state);
}

fn cost_search_paths(paths: &Vec<Path>, goal: i32, start_pos: Vec<char>) -> State {
    let mut node = State { pos: start_pos.clone(), keys: HashSet::from_iter(start_pos.clone()), cost: 0 };
    //node.keys.insert('@');

    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    frontier.push(node);
    loop {
        frontier.sort_by(|s, o| s.cost.cmp(&o.cost));
        if frontier.len() < 1 {
            break;
        }
        let state = frontier.remove(0);
        println!("{:?}", &state);
        // println!("frontier: {:?}", frontier);
        // println!("explored: {:?}", explored);
        let mut keyvec = state.keys.clone().into_iter().collect_vec();
        keyvec.sort();
        if explored.contains(&(state.pos.clone(), keyvec.clone())) {
            continue;
        }
        explored.insert((state.pos.clone(), keyvec));

        // Check if goal is met
        if state.keys.len() as i32 == goal {
            return state;
        }

        // Get next states
        // 'outer: for p in paths.iter().filter(|p| p.a == state.pos || p.b == state.pos) {
        'outer: for p in paths.iter().filter(|p| state.pos.contains(&p.a) || state.pos.contains(&p.b)) {
            for k in &p.keys {
                if !state.keys.contains(k) {
                    // println!("\t\tSkip because key not found: {} {:?} {:?}", k, &state.keys, &p);
                    continue 'outer;
                }
            }
            let mut pos = state.pos.clone();
            let mut ind = 0;
            let mut c = ' ';
            for i in 0..pos.len() {
                if &pos[i] == &p.a {
                    ind = i;
                    c = p.b;
                    break;
                } else if &pos[i] == &p.b {
                    ind = i;
                    c = p.a;
                    break;
                }
            }
            /*let c = if state.pos.contains(&p.a) {
                p.b
            } else {
                p.a
            };*/
            if state.keys.contains(&c) {
                // println!("\t\tSkip because key already obtained: {} {:?} {:?}", c, &state, &p);
                continue;
            }
            let mut k = state.keys.clone();
            k.insert(c);
            let mut keyvec = k.clone().into_iter().collect_vec();
            keyvec.sort();
            pos.remove(ind);
            pos.push(c);
            pos.sort();
            if explored.contains(&(pos.clone(), keyvec)) {
                // println!("\t\tSkip because already explored: {} {:?} {:?}", c, &state, &p);
                continue;
            }
            let next_state = State {pos: pos, cost: state.cost + p.cost, keys: k};
            // println!("New frontier: {:?}", &next_state);
            frontier.push(next_state);
        }
        //let next_states = get_next_states(&map, &state, &goals);
    }
    return State {pos: vec!['@'], keys: HashSet::new(), cost: 0};
}

fn get_all_paths(map: &HashMap<(i32, i32), char>) -> Vec<Path> {
    let mut paths = Vec::new();

    let mut keys = HashSet::new();
    for (_, v) in map {
        if v >= &'a' && v <= &'z' {
            keys.insert(*v);
        }
    }

    for a in 'a'..='z' {
        for b in vec!['@', '0', '1', '2', '3'] {
            let p = get_paths_a_to_b_new(map, b, a, keys.clone());
            paths.extend(p);
        }
    }

    for a in 'a'..='z' {
        for b in ((a as u8 + 1) as char)..='z' {
            let p = get_paths_a_to_b_new(map, a, b, keys.clone());
            paths.extend(p);
        }
    }

    for p in &paths {
        println!("{} to {} path: {:?}", p.a, p.b, p);
    }

    return paths;
}

fn get_location(map: &HashMap<(i32, i32), char>, c: char) -> Option<(i32, i32)> {
    for (k, v) in map {
        if v == &c {
            return Some(k.clone());
        }
    }
    return None;
}

fn get_paths_a_to_b_new(map: &HashMap<(i32, i32), char>, a: char, b: char, keys: HashSet<char>) -> Vec<Path> {
    let loc_a = get_location(map, a);
    let loc_b = get_location(map, b);
    let mut paths = Vec::new();

    if loc_a.is_none() || loc_b.is_none() { return paths; };

    let path = ucs_with_keys(map, &loc_a.unwrap(), &loc_b.unwrap(), &keys);
    //println!("{} {:?} to {} {:?} path: {:?}", a, loc_a.unwrap(), b, loc_b.unwrap(), path);

    if path.is_some() {
        let p = path.unwrap();
        paths.push(p.clone());

        for v in &p.keys {
            let mut k = keys.clone();
            k.remove(v);
            let more_paths = get_paths_a_to_b_new(map, a, b, k);
            paths.extend(more_paths);
        }
    }
    return paths;
}

fn ucs_with_keys(map: &HashMap<(i32, i32), char>, start: &(i32, i32), end: &(i32, i32), keys: &HashSet<char>) -> Option<Path> {
    let mut node = start.clone();
    let mut cost = 0;
    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    let mut min_cost = HashMap::new();
    let mut door = HashSet::new();
    let mut doors = HashMap::new();
    frontier.push(node.clone());
    min_cost.insert(node.clone(), cost.clone());
    doors.insert(node.clone(), door.clone());
    loop {
        // println!("Frontier: {:?}", &frontier);
        // Return if frontier is empty or there are no goals
        if frontier.is_empty() {
            return None;
        }
        // If node is goal then add to output and remove from goals list
        node = frontier.remove(0);
        // dbg!(&frontier, &node, &min_cost);
        cost = min_cost.get(&node).unwrap().clone();
        door = doors.get(&node).unwrap().clone();

        //let val = map.get(&node).unwrap_or(&' ');
        // dbg!(val);
        if &node == end {
            return Some(Path {
                a: map.get(start).unwrap().clone(),
                b: map.get(end).unwrap().clone(),
                cost: cost,
                keys: door,
            });
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
            // dbg!(&frontier, &explored, &next);
            //println!("{:?}", map.get(&next).unwrap_or(&'#'));
            let val = map.get(&next).unwrap_or(&' ');

            if frontier.contains(&next)
               || explored.contains(&next)
                || val == &'#' {
                continue;
            }
            if val >= &'A' && val <= &'Z' {
                if !keys.contains(&val.to_ascii_lowercase()) {
                    continue;
                } else {
                    door.insert(val.clone().to_ascii_lowercase());
                }
            }

            frontier.push(next);
            min_cost.insert(next.clone(), cost + 1);
            doors.insert(next.clone(), door.clone());
                // println!("frontier: {:?} mincost: {:?}", frontier, min_cost);
        }
    }
}

/*fn cost_search(map: &HashMap<(i32, i32), char>) {
    let mut pos = get_pos(&map);
    let goals = get_goals(&map);
    let mut frontier = Vec::new();
    frontier.push(State{pos: pos.clone(), keys: HashSet::new(), cost: 0});
    let mut explored = Vec::new();
    let mut cnt = 0;
    loop {
        cnt += 1;
        frontier.sort_by(|s, o| s.cost.cmp(&o.cost));
        let state = frontier.remove(0);
        // Get next states
        let next_states = get_next_states(&map, &state, &goals);
        //println!("states: {:?}", state);
        if cnt % 1000 == 0 {
            for f in 0..min(frontier.len(), 10) {
                print!("{:?} ", frontier.get(f).unwrap());
            }
            println!();
        }
        if next_states.is_empty() {
            println!("Final state: {:?}", state);
            break
        }
        explored.push(state);
        'outer: for s in next_states {
           /* for mut e in &mut explored {
                //println!("a: {:?} b: {:?} diff: {:?}", s.keys, e.keys, s.keys.symmetric_difference(&e.keys));
                if s.pos.0 == e.pos.0 && s.pos.1 == e.pos.1 && s.keys.symmetric_difference(&e.keys).count() == 0 {
                    if s.cost < e.cost {
                        e.cost = s.cost;
                    } else {
                        // println!("Skip");
                        continue 'outer;
                    }
                }
            }*/
            frontier.push(s);
        }
    }
}
*/
/*fn get_next_states(map: &HashMap<(i32, i32), char>, state: &State, goals: &HashSet<char>) -> Vec<State> {
    let mut out = Vec::new();
    let mut goals = goals.clone();
    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    let mut min_cost = HashMap::new();
    frontier.push(state.pos.clone());
    min_cost.insert(state.pos.clone(), state.cost);
    loop {
        if frontier.is_empty() || goals.is_empty() {
            return out;
        }
        let node = frontier.remove(0);
        explored.insert(node.clone());
        let cost = *min_cost.get(&node).unwrap();
        let val = map.get(&node).unwrap_or(&' ').clone();
        if val >= 'a' && val <= 'z' && goals.contains(&val) && !state.keys.contains(&val) {
            let mut keys = state.keys.clone();
            keys.insert(val);
            out.push(State{pos: node, keys: keys, cost: cost});
        } else {
            for i in 0..4 {
                let next = match i {
                    0 => (node.0, node.1 + 1),
                    1 => (node.0, node.1 - 1),
                    2 => (node.0 - 1, node.1),
                    3 => (node.0 + 1, node.1),
                    _ => (0, 0),
                };
                // dbg!(&frontier, &explored, &next);
                //println!("{:?}", map.get(&next).unwrap_or(&'#'));
                let val = map.get(&next).unwrap_or(&' ');
                if !frontier.contains(&next)
                    && !explored.contains(&next)
                    && val != &'#'
                    && (!(val >= &'A' && val <= &'Z') || state.keys.contains(&val.to_ascii_lowercase())) {
                    frontier.push(next);
                    min_cost.insert(next.clone(), cost + 1);
                    // println!("frontier: {:?} mincost: {:?}", frontier, min_cost);
                }
            }
        }
    }
}

fn uniform_cost_search(start: &(i32, i32), goals: &HashSet<char>, map: &HashMap<(i32, i32), char>)
                       -> Vec<(i32, (i32, i32), char)> {
    let mut out = Vec::new(); // Steps, Position, Character
    let mut node = start.clone();
    let mut cost = 0;
    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    let mut min_cost = HashMap::new();
    frontier.push(node.clone());
    min_cost.insert(node.clone(), cost.clone());
    loop {
        // println!("Frontier: {:?}", &frontier);
        // Return if frontier is empty or there are no goals
        if frontier.is_empty() || goals.is_empty() {
            return out;
        }
        // If node is goal then add to output and remove from goals list
        node = frontier.remove(0);
        // dbg!(&frontier, &node, &min_cost);
        cost = min_cost.get(&node).unwrap().clone();
        let val = map.get(&node).unwrap_or(&' ');
        // dbg!(val);
        if goals.contains(val) {
            out.push((cost.clone(), node.clone(), val.clone()));
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
            // dbg!(&frontier, &explored, &next);
            //println!("{:?}", map.get(&next).unwrap_or(&'#'));
            let val = map.get(&next).unwrap_or(&' ');
            if !frontier.contains(&next)
                && !explored.contains(&next)
                && val != &'#'
                && !(val >= &'A' && val <= &'Z') {
                frontier.push(next);
                min_cost.insert(next.clone(), cost + 1);
                // println!("frontier: {:?} mincost: {:?}", frontier, min_cost);
            }
        }
    }
}

fn count_steps(map: &HashMap<(i32, i32), char>, pos: &(i32, i32), goals: &HashSet<char>, mut best_cost: i32, self_cost: i32, level: i32) -> i32 {
    if goals.is_empty() {
        return 0;
    }
    // println!("Best cost: {}", best_cost);
    let mut map = map.clone();
    let mut goals = goals.clone();
    let costs = uniform_cost_search(pos, &goals, &map);
    let mut min_cost = i32::max_value();
    for c in &costs {
        if (self_cost + c.0) >= best_cost {
            // println!("Skip");
            continue;
        }
        let mut next_map = map.clone();
        let mut next_goals = goals.clone();
        next_map.remove(&c.1);
        next_goals.remove(&c.2);
        let door = c.2.to_ascii_uppercase();
        for (k, v) in &map {
            if v == &door {
                next_map.remove(k);
                break;
            }
        }
        let cs = count_steps(&next_map, &c.1, &next_goals, best_cost, self_cost + c.0, level + 1);
        if cs >= best_cost { // Avoid overflow
            continue;
        }
        let next_cost = c.0 + cs;
        if next_cost < min_cost {
            min_cost = next_cost;
        }
        if self_cost + next_cost < best_cost {
            best_cost = self_cost + next_cost;
            println!("Best cost: {}", best_cost);
        }
    }
    // println!("{:?}", &costs);
    //  println!("Min cost: {}", min_cost);
    if level < 11 {
        println!("level: {}", level);
    }
    return min_cost;
}
*/
fn print_map(map: &HashMap<(i32, i32), char>) {
    let mut x = (0, 0);
    let mut y = (0, 0);
    for ((xi, yi), _) in map {
        if xi < &x.0 {
            x.0 = *xi;
        }
        if xi > &x.1 {
            x.1 = *xi;
        }
        if yi < &y.0 {
            y.0 = *yi;
        }
        if yi > &y.1 {
            y.1 = *yi;
        }
    }
    for yi in y.0..=y.1 {
        for xi in x.0..=x.1 {
            print!("{}", map.get(&(xi, yi)).unwrap_or(&'.'));
        }
        println!();
    }
}

fn replace_char(map: &mut HashMap<(i32, i32), char>, old: char, new: char) {
    let mut locs = Vec::new();
    for (k, v) in &*map {
        if v == &old {
            locs.push(k.clone());
        }
    }
    for k in locs {
        map.insert(k, new);
    }
}

fn get_pos(map: &HashMap<(i32, i32), char>) -> (i32, i32) {
    for (k, v) in map {
        if v == &'@' {
            return *k;
        }
    }
    return (0,0);
}

fn get_goals(map: &HashMap<(i32, i32), char>) -> HashSet<char> {
    let mut goals = HashSet::new();
    for (_,v) in map {
        if v >= &'a' && v <= &'z' {
            goals.insert(*v);
        }
    }
    return goals;
}

fn load(fname: &str) -> HashMap<(i32, i32), char> {
    let input = std::fs::read_to_string(fname).unwrap();
    let mut map = HashMap::new();
    let mut x;
    let mut y = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            if c != '.' {
                map.insert((x,y), c);
            }
            x += 1;
        }
        y += 1;
    }
    return map;
}