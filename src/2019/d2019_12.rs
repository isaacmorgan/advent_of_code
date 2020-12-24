use std::collections::HashSet;

const FNAME: &str = "./input/2019/2019-12.txt";

pub fn main() {
    let mut planets = load();
    let mut e = Vec::new();
    println!("{:?}", planets);
    for i in 0..1_000 {
        step(&mut planets);
        //println!("Cnt: {}", i+1);
        //println!("{:?}", planets);
        e = energy(&planets);
    }
    println!("energy: {:?}\ntotal energy: {}", e, e.iter().sum::<i64>());

    let xcnt = count_loop(1);
    println!("xloop: {}", xcnt);
    let ycnt = count_loop(2);
    println!("yloop: {}", ycnt);
    let zcnt = count_loop(3);
    println!("zloop: {}", zcnt);
    println!("gcf: {}", gcf(&vec![xcnt, ycnt, zcnt]));
    println!("lcm: {}", lcm(&vec![xcnt, ycnt, zcnt]));
    let steps_to_repeat = lcm(&vec![xcnt, ycnt, zcnt]);
    println!("Steps until repeat: {}", steps_to_repeat)
}

fn lcm(nums: &Vec<i64>) -> i64 {
    if nums.len() == 2 {
        nums[0]*nums[1]/gcf(&nums)
    } else {
        let tmp = nums.split_last().unwrap();
        lcm(&vec![lcm(&tmp.1.to_vec()), *tmp.0])
    }
}

fn gcf(nums: &Vec<i64>) -> i64 {
    let nmax = (nums.iter().min().unwrap())/2;
    'outer: for i in (0..=nmax).rev() {
        for n in nums {
            if (n/i)*i != *n {
                continue 'outer;
            }
        }
        return i;
    }
    return 0;
}

fn count_loop(ind: usize) -> i64 {
    let mut setx = HashSet::new();
    let mut planets = load();
    setx.insert(get_axis_code(&planets, ind));
    let mut xcnt = 0;
    loop {
        step(&mut planets);
        xcnt += 1;
        let code = get_axis_code(&planets, ind);
        if setx.contains(&code) {
            break;
        } else {
            setx.insert(code);
        }
    }
    return xcnt;
}

fn get_axis_code(planets: &Vec<[i64; 7]>, ind: usize) -> Vec<i64> {
    let mut out = Vec::new();
    for p in planets {
        out.push(p[ind]);
        out.push(p[ind + 3]);
    }
    return out;
}

fn energy(planets: &Vec<[i64; 7]>) -> Vec<i64> {
    planets.iter().map(|p| (p[1].abs() + p[2].abs() + p[3].abs())*(p[4].abs() + p[5].abs() + p[6].abs())).collect()
}

fn step(planets: &mut Vec<[i64; 7]>) {
    for i in 0..planets.len() {
        for j in 0..planets.len() {
            if i == j {
                continue;
            }
            for k in 1..=3 {
                let b = planets.get(j).unwrap();
                let bk = b[k];
                let a = planets.get_mut(i).unwrap();
                if a[k] > bk {
                    a[k+3] -= 1;
                } else if a[k] < bk {
                    a[k+3] += 1;
                }
            }
        }
    }

    for a in planets {
        for i in 1..=3 {
            a[i] += a[i+3];
        }
    }
}

fn load() -> Vec<[i64; 7]>{
    let input = std::fs::read_to_string(FNAME).unwrap();
    let re = regex::Regex::new(r"<x=([\d-]+), y=([\d-]+), z=([\d-]+)>").unwrap();
    let mut out = Vec::new();
    let mut id = 0;
    for cap in re.captures_iter(&input) {
        out.push([id, cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap(), 0, 0, 0]);
        id += 1;
    }
    return out;
}
