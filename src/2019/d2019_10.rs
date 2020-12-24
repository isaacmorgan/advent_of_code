use std::collections::HashSet;

const FNAME: &str = "./input/2019/2019-10.txt";

pub fn main() {
    let map = load();
    println!("{:?}", map);

    let count = count_visible_neighbors(&map);
    println!("{:?}", count);

    let max_c = count.iter().enumerate().max_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap();
    println!("Best location is {:?} with {} asteroids detected.", map.get(max_c.0).unwrap(), max_c.1);

    let ra = get_asteroid_range_angle(&map, &map.get(max_c.0).unwrap());
    println!("Range-Angle: {:?}", ra.get(max_c.0).unwrap());

    let burn_order = spin_and_destroy(&ra, &map);
    println!("200th asteroid: {:?}\n\tcode: {}", burn_order[199], burn_order[199][1] + 100*burn_order[199][0] );
}

fn spin_and_destroy(ra: &Vec<[i32; 2]>, map: &Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    let mut burn_order = Vec::new();
    let mut ra = ra.clone();
    let mut map = map.clone();
    let mut angle = -1;
    let self_ind = ra.iter().position(|x| x[0] == 0).unwrap();
    ra.remove(self_ind);
    map.remove(self_ind);

    loop {
        //println!("ra: {:?}\nangle: {}", ra, angle);
        // Find next ra
        let mut valid = false;
        let mut ind = 0;
        let mut val = [0, 0];
        let mut minval = i32::max_value();
        for (i, ira) in ra.iter().enumerate() {
            if ira[1] > angle && ira[1] <= minval {
                ind = i;
                val = ira.clone();
                minval = val[1];
                valid = true;
                //println!("Choice: ind: {} val: {:?}", i, val);
            }
        }
        if !valid {
            angle = -1;
            continue;
        }
        let next_asteroid = val;
        //println!("{}th asteroid: {:?} at {:?}", burn_order.len() + 1, next_asteroid, map.get(ind).unwrap());
        ra.remove(ind);
        burn_order.push(map.remove(ind));
        angle = next_asteroid[1];
        if ra.is_empty() {
            break;
        }
    }
    return burn_order;
}

fn get_asteroid_range_angle(map: &Vec<[i32; 2]>, origin: &[i32; 2]) -> Vec<[i32; 2]>{
    let mut ra = Vec::new();
    for (i, m) in map.iter().enumerate() {
        let dx = origin[0] - m[0];
        let dy = origin[1] - m[1];
        let angle = ((((dy as f32).atan2(dx as f32) * 1000.0 - 3141.59265/2.0) + 4.0*3141.59265) as i32) % 6283;
        let r = dx*dx + dy*dy;
        ra.push([r, angle]);
    }
    return ra;
}

fn count_visible_neighbors(map: &Vec<[i32; 2]>) -> Vec<i32> {
    let mut visible_count = Vec::new();
    for m in map {
        let mut set = HashSet::new();
        let mut cnt = 0;
        for n in map {
            if m == n {
                continue;
            }
            let dx = n[0] - m[0];
            let dy = n[1] - m[1];
            let angle = ((dy as f32).atan2(dx as f32) * 1000.0) as i32;
            //println!("x: {} y: {} a: {}", dx, dy, angle);
            if !set.contains(&angle) {
                set.insert(angle);
                cnt += 1;
            }
        }
        visible_count.push(cnt);
    }

    return visible_count;
}

fn load() -> Vec<[i32; 2]> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    let mut map = Vec::new();
    let mut row = 0;
    let mut col = 0;
    for line in input.split("\n") {
        col = 0;
        for c in line.chars() {
            if c == '#' {
                map.push([col, row]);
            }
            col += 1;
        }
        row += 1;
    }
    return map;
}