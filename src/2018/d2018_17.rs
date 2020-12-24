use std::fs;
use std::fs::File;
use std::io::Write;
use regex::Regex;
use std::cmp::max;

const WATER: i32 = 2;
const WET: i32 = 3;
const SAND: i32 = 0;
const CLAY: i32 = 1;

const NX: usize = 2000;
const NY: usize = 2000;

const FNAME0: &str = "./input/2018-17-0.txt";
const FNAME: &str = "./input/2018-17.txt";
const FOUT: &str = "./input/2018-17-out.txt";

pub fn main() {
    let mut map = load_input();
    //print_map(&map);
    step(500, 0, &mut map);
    let ylim = get_y_lim(&map);
    let cnt = count(ylim, &map);
    println!("Wet tiles: {}", cnt);
    println!("{:?}", ylim);
    //print_map(&map);

/*    let mut file = fs::File::create(FOUT).unwrap();
    for a in map {
        write!(file, "\n");
        for b in a {
            write!(file, "{}", match b {
                SAND => '.',
                CLAY => '#',
                WATER => '~',
                WET => '|',
                _ => ' '
            }.to_string());
        }
    }
    write!(file, "\n");*/
}

fn get_y_lim(map: &Vec<Vec<i32>>) -> [usize; 2] {
    let mut ylim = [0; 2];
    let mut this_y = 0;
    for y in map {
        for x in y {
            if x == &CLAY {
                if ylim[0] == 0 {
                    ylim[0] = this_y;
                }
                ylim[1] = this_y;
                continue;
            }
        }
        this_y += 1;
    }
    return ylim;
}

fn count(ylim: [usize; 2], map: &Vec<Vec<i32>>) -> i32{
    let mut cnt = 0;
        for y in ylim[0]..=ylim[1] {
            for x in 0..NX {
               match map[y][x] {
               WATER => cnt += 1,
               _ => (),
           }
        }
        //println!("line: {} count: {}", y, cnt);
    }
    return cnt;
}

fn step(mut x: usize, mut y: usize, map: &mut Vec<Vec<i32>>) {
    //print_map_segment([max(0, x as i32 - 50) as usize, max(0, x as i32 + 50) as usize],
    //                  [max(0, y as i32 - 50) as usize, max(0, y as i32 + 50) as usize], x, y, map);
    let x0 = x;
    let y0 = y;

    // move down as far as possible
    let mut j = y;
    while (y + 1 < NY) && (map[y + 1][x] == SAND || map[y + 1][x] == WET ) {
        //println!("Move down");
        map[y][x] = WET;
        y += 1;
    }

    // if off screen, stop
    if y + 1 == NY {
        //println!("y + 1 == NY");
        return
    }

    // fill left-right if possible
    // if filled, step up 1 and try to fill lr again
    while fill_lr(x, y, map) {
        y -= 1;
    }

    // if not filled, find overflow spots and call step with new initial position
    let a = check_overflow_left(x, y, map);
    let b = check_overflow_right(x, y, map);

    if a[0] == 1 {
        println!("Start again left at x: {} y: {}, from {} {}", a[1], y, x0, y0);
        //print_map_segment([max(0, a[1] as i32 - 50) as usize, max(0, a[1] as i32 + 50) as usize],
        //                  [max(0, y as i32 - 50) as usize, max(0, y as i32 + 50) as usize], map);
        step(a[1], y, map);
    }
    if b[0] == 1 {
        println!("Start again right at x: {} y: {}, from {} {}", b[1], y, x0, y0);
        //print_map_segment([max(0, b[1] as i32 - 50) as usize, max(0, b[1] as i32 + 50) as usize],
        //                  [max(0, y as i32 - 50) as usize, max(0, y as i32 + 50) as usize], map);
        step(b[1], y, map);
    }
}

fn check_overflow_left(x: usize, y: usize, map: &mut Vec<Vec<i32>>) -> [usize; 2] {
    println!("Check of left at {}, {}", x, y);
    let mut a = x;

    while (map[y+1][a] == WATER || map[y+1][a] == CLAY) && (map[y][a-1] != CLAY) {
        a -= 1;
    }

    for i in a..=x {
        map[y][i] = WET;
    }

    if (map[y+1][a] != WATER) && (map[y+1][a] != CLAY) &&  (map[y+1][a] != WET) {
        return [1, a];
    }

    return [0, a];
}

fn check_overflow_right(x: usize, y: usize, map: &mut Vec<Vec<i32>>) -> [usize; 2] {
    println!("Check of right at {}, {}", x, y);
    let mut a = x;

    while (map[y+1][a] == WATER || map[y+1][a] == CLAY) && map[y][a+1] != CLAY {
        a += 1;
    }

    for i in x..=a {
        map[y][i] = WET;
    }

    if (map[y+1][a] != WATER) && (map[y+1][a] != CLAY)  &&  (map[y+1][a] != WET){
        return [1, a];
    }

    return [0, a];
}

fn check_overflow(x: usize, y: usize, map: &mut Vec<Vec<i32>>) {
    let mut a = x;
    let mut b = x;

    if map[y][a] != WATER {
        map[y][a] = WET;
    }

    // Check left
    loop {
        if ((map[y+1][a] == WATER) || (map[y+1][a] == CLAY)) && (map[y][a-1] != CLAY) {
            a -= 1;
            if map[y][a] != WATER {
                map[y][a] = WET;
            }
        } else if ((map[y+1][a] == WATER) || (map[y+1][a] == CLAY)) && (map[y][a-1] == CLAY) {
            break;
        } else {
            if a != x {
                map[y][a] = WET;
                println!("Start again at x: {} y: {}", a, y+1);
                //print_map_segment([max(0, a as i32 - 50) as usize, max(0, a as i32 + 50) as usize],
                //                  [max(0, y as i32 - 50) as usize, max(0, y as i32 + 50) as usize], map);
                step(a, y+1, map);
            }
            break;
        }
    }

    // Check right
    loop {
        if ((map[y+1][b] == WATER) || (map[y+1][b] == CLAY)) && (map[y][b+1] != CLAY) {
            b += 1;
            if map[y][b] != WATER {
                map[y][b] = WET;
            }
        } else if ((map[y+1][b] == WATER) || (map[y+1][b] == CLAY)) && (map[y][b+1] == CLAY) {
            break;
        } else {
            if b != x {
                map[y][b] = WET;
                println!("Start again at x: {} y: {}", b, y+1);
                //print_map_segment([max(0, b as i32 - 50) as usize, max(0, b as i32 + 50) as usize],
                //                  [max(0, y as i32 - 50) as usize, max(0, y as i32 + 50) as usize], map);
                step(b, y+1, map);
            }
            break;
        }
    }
}

fn fill_lr(x: usize, y: usize, map: &mut Vec<Vec<i32>>) -> bool {
    let mut a = x;
    let mut b = x;

    // Check left
    loop {
        if ((map[y+1][a] == WATER) || (map[y+1][a] == CLAY)) && (map[y][a-1] != CLAY) {
            //println!("Check left");
            a -= 1;
        } else if ((map[y+1][a] == WATER) || (map[y+1][a] == CLAY)) && (map[y][a-1] == CLAY) {
            break;
        } else {
            return false;
        }
    }

    // Check right
    loop {
        if ((map[y+1][b] == WATER) || (map[y+1][b] == CLAY)) && (map[y][b+1] != CLAY) {
            //println!("Check right");
            b += 1;
        } else if ((map[y+1][b] == WATER) || (map[y+1][b] == CLAY)) && (map[y][b+1] == CLAY) {
            break;
        } else {
            //println!("Failed check right");
            return false;
        }
    }

    for i in a..=b {
        map[y][i] = WATER;
    }

    return true;
}

fn print_map_segment(x_b: [usize; 2], y_b: [usize; 2], x0: usize, y0: usize, map: &Vec<Vec<i32>>) {
    for y in y_b[0]..=y_b[1] {
        for x in x_b[0]..=x_b[1] {
            if y == y0 && x == x0 {
                print!("{}", "&");
            } else if y < NY && x < NX {
                print!("{}", match map[y][x] {
                    SAND => '.',
                    CLAY => '#',
                    WATER => '~',
                    WET => '|',
                    _ => ' '
                });
            }
        }
        println!();
    }
    println!();
}

fn print_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for col in row {
            print!("{}", match *col {
                SAND => '.',
                CLAY => '#',
                WATER => '~',
                WET => '|',
                _ => ' '
            });
        }
        println!();
    }
    println!();
}

fn load_input() -> Vec<Vec<i32>> {
    let mut map = vec![vec![0; NX]; NY];
    let contents = fs::read_to_string(FNAME).expect("Error reading file");
    let lines = contents.split("\n");
    for line in lines {
        println!("{}", line);
        parse(line,&mut map);
    }
    return map;
}

fn parse(line: &str, map: &mut Vec<Vec<i32>>) {
    let re = Regex::new(r"^([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)").unwrap();
    for cap in re.captures_iter(line) {
        let a = cap[4].parse::<usize>().unwrap();
        let b = cap[5].parse::<usize>().unwrap();
        match &cap[1] {
            "x" => {
                let j = cap[2].parse::<usize>().unwrap();
                for i in a..=b {
                    map[i][j] = CLAY;
                }
            },
            "y" => {
                let i = cap[2].parse::<usize>().unwrap();
                for j in a..=b {
                    map[i][j] = CLAY;
                }
            }
            _ => (),
        }
    }
}
