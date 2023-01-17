use std::collections::HashSet;
use crate::tools;
static FNAME: &str = "./input/2022/2022-18-01.txt";

pub fn main() {
    let input = tools::load(FNAME);
    part1(&input);
    part2(&input);
}

fn part1(lines: &Vec<String>) {
    let cubes = parse(&lines);
    let mut faces = 0;
    for c in &cubes {
        faces += count_faces(&cubes, &c);
    }

    println!("Part 1 total faces: {faces}");
}

fn part2(lines: &Vec<String>) {
    let cubes = parse(&lines);
    let mut clouds = HashSet::new();
    let x_min = cubes.iter().map(|(x,y,z)| x).min().unwrap() - 1;
    let x_max = cubes.iter().map(|(x,y,z)| x).max().unwrap() + 1;
    let y_min = cubes.iter().map(|(x,y,z)| y).min().unwrap() - 1;
    let y_max = cubes.iter().map(|(x,y,z)| y).max().unwrap() + 1;
    let z_min = cubes.iter().map(|(x,y,z)| z).min().unwrap() - 1;
    let z_max = cubes.iter().map(|(x,y,z)| z).max().unwrap() + 1;

    let mut num_clouds = clouds.len();

    loop {
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    if x == x_min || x == x_max || y == y_min || y == y_max || z == z_min || z == z_max {
                        clouds.insert((x, y, z));
                        continue;
                    }
                    if cubes.contains(&(x, y, z)) {
                        continue;
                    }
                    if touches_cloud(&clouds, &(x, y, z)) {
                        clouds.insert((x, y, z));
                    }
                }
            }
        }
        if num_clouds == clouds.len() {
            break;
        } else {
            num_clouds = clouds.len();
        }
    }

    let mut faces = 0;
    for c in cubes {
        faces += count_cloud_faces(&clouds, &c);
    }
    println!("Part 2 total faces: {faces}");
}

fn count_cloud_faces(clouds: &HashSet<(i32, i32, i32)>, cube: &(i32, i32, i32)) -> i32 {
    let mut cnt = 0;
    for dxyz in [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
        if clouds.contains(&(cube.0 + dxyz.0, cube.1 + dxyz.1, cube.2 + dxyz.2)) {
            cnt += 1;
        }
    }
    return cnt;
}

fn touches_cloud(clouds: &HashSet<(i32, i32, i32)>, cube: &(i32, i32, i32)) -> bool {
    for dxyz in [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
        if clouds.contains(&(cube.0 + dxyz.0, cube.1 + dxyz.1, cube.2 + dxyz.2)) {
            return true;
        }
    }
    return false;
}

fn count_faces(set: &HashSet<(i32, i32, i32)>, cube: &(i32, i32, i32)) -> i32 {
    let mut cnt = 0;
    for dxyz in [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
        if !set.contains(&(cube.0 + dxyz.0, cube.1 + dxyz.1, cube.2 + dxyz.2)) {
            cnt += 1;
        }
    }
    return cnt;
}

fn parse(lines: &Vec<String>) -> HashSet<(i32, i32, i32)> {
    let mut cubes = HashSet::new();
    for ln in lines {
        let (a, rem) = ln.split_once(",").unwrap();
        let (b,c) = rem.split_once(",").unwrap();
        cubes.insert((a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()));
    }
    return cubes;
}
