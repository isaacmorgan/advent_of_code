use std::fs;
use std::collections::VecDeque;
use std::cmp::max;

const N: usize = 500;
const X0: usize = N/2;
const Y0: usize = N/2;
const UNK: i32 = 0;
const ROOM: i32 = 1;
const DOOR: i32 = 2;
const WALL: i32 = 3;

const OFFSET: i32 = 10;

const FNAME: &str = "./input/2018/2018-20.txt";

struct Room {
    x: usize,
    y: usize,
    d: i32,
}

pub fn main() {
    let br = "(EESS)(WNSE|)SSS|WWWSSSSE((SW|NNNE))";
    println!("{}", br);
    //let a = branch_options(br);
    let a = split_branches(&br.to_string());
    //let b = sequence_list("WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))".to_string());
    let b = split_sequences(&br.to_string());
    println!("{:?}", a);
    println!("{:?}", b);

    let mut re = load_str();
    re = re[1..re.len()-1].to_string();
    let mut map = new_map();
    map[X0][Y0] = ROOM;
    //step_new(X0, Y0, &mut map, &"WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))".to_string());
    step_new(X0, Y0, &mut map, &re);
    print_map([X0-25, X0+25], [Y0-25, Y0+25], &map);

    unk_to_wall(&mut map);

    let mut room_list = VecDeque::new();
    room_list.push_back(Room {
        x: X0, y: Y0, d: OFFSET,
    });

    loop {
        if room_list.is_empty() {
            break;
        }
        let room = room_list.pop_front().unwrap();
        flood_fill(room.x, room.y, room.d, &mut map, &mut room_list);
    }

    let mut maxval = 0;
    let mut cnt_1k = 0;
    for x in 0..N {
        for y in 0..N {
            if map[x][y] > maxval {
                maxval = map[x][y];
            }
            if map[x][y] >= (1000 + OFFSET) {
                cnt_1k += 1;
            }
        }
    }
    maxval = maxval - OFFSET;
    print_map([X0-25, X0+25], [Y0-25, Y0+25], &map);
    println!("Max Room Distance: {}", maxval);
    println!("Rooms 1,000 Steps Away: {}", cnt_1k);
}

fn flood_fill(x: usize, y: usize, d: i32, map: &mut Vec<Vec<i32>>, room_list: &mut VecDeque<Room>) {
    //println!("x: {} y: {} d: {} map: {}", x, y, d, map[x][y]);

    // Check self and compare d to own spot
    if map[x][y] == UNK || map[x][y] == DOOR {
        println!("Error!")
    } else if map[x][y] == WALL {
        return;
    }

    // If the spot is still labeled ROOM, set it to d.
    // Otherwise, if d is greater or =, return
    if map[x][y] == ROOM {
        map[x][y] = d;
    } else if d >= map[x][y] {
        return;
    }

    // else, overwrite d and recursively call flood_fill on all neighbors
    map[x][y] = d;
    if map[x-1][y] == DOOR {
        room_list.push_back(Room {
            x: x-2, y: y, d: d+1,
        });
        //flood_fill(x - 2, y, d + 1, map);
    }
    if map[x+1][y] == DOOR {
        room_list.push_back(Room {
            x: x+2, y: y, d: d+1,
        });
        //flood_fill(x + 2, y, d + 1, map);
    }
    if map[x][y-1] == DOOR {
        room_list.push_back(Room {
            x: x, y: y-2, d: d+1,
        });
        //flood_fill(x, y-2, d + 1, map);
    }
    if map[x][y+1] == DOOR {
        room_list.push_back(Room {
            x: x, y: y+2, d: d+1,
        });
        //flood_fill(x, y+2, d + 1, map);
    }
}

fn unk_to_wall(map: &mut Vec<Vec<i32>>) {
    for x in 0..N {
        for y in 0..N {
            if map[x][y] == UNK {
                map[x][y] = WALL;
            }
        }
    }
}

fn load_str() -> String {
    let mut contents = fs::read_to_string(FNAME).expect("Error reading file");
    return contents.split("\n").next().unwrap().to_string();
}

fn print_map(xlim: [usize; 2], ylim: [usize; 2], map: &Vec<Vec<i32>>) {
    for y in ylim[0]..ylim[1] {
        for x in xlim[0]..xlim[1] {
            print!("{}", match map[x][y] {
                UNK => '?',
                ROOM => '.',
                DOOR => if y%2==0 {'|'} else {'-'},
                WALL => '#',
                _ => (map[x][y] - OFFSET).to_string().chars().next().unwrap(),
            })
        }
        println!()
    }
    println!()
}

fn step_new(mut x: usize, mut y: usize, map: &mut Vec<Vec<i32>>, s: &String) {
    // Split into sequences
    let sequences = split_sequences(s);
    println!("Sequences: {:?}", sequences);

    // Loop over each sequence
    for se in sequences {
        // If sequence begins with parenthesis it's a branch, otherwise it's an executable sequence
        if se.starts_with('(') {
            // Branch
            let branches = split_branches(&se);
            for br in branches {
                step_new(x, y, map, &br);
            }
        } else {
            // Perform sequence
            for c in se.chars().into_iter() {
                match c {
                    'N' => {
                        map[x][y-1] = DOOR;
                        map[x][y-2] = ROOM;
                        y = y - 2;
                    },
                    'S' => {
                        map[x][y+1] = DOOR;
                        map[x][y+2] = ROOM;
                        y = y + 2;
                    },
                    'E' => {
                        map[x + 1][y] = DOOR;
                        map[x + 2][y] = ROOM;
                        x = x + 2;
                    },
                    'W' => {
                        map[x - 1][y] = DOOR;
                        map[x - 2][y] = ROOM;
                        x = x - 2;
                    },
                    _ => println!("Very bad: '{}' : {:?}", c, se),
                }
            }
        }
    }
}

fn new_map() -> Vec<Vec<i32>> {
    vec![vec![UNK; N]; N]
}

fn split_sequences(s: &String) -> Vec<String> {
    let mut sequences = Vec::new();

    let mut level = 0;
    let mut split_inds = vec!(0);
    let mut i = 0;
    for c in s.chars() {
        match c {
            '(' => {
                level += 1;
                if level == 1 && !split_inds.contains(&i) {
                    split_inds.push(i);
                }
            },
            ')' => {
                level -= 1;
                if level == 0 {
                    split_inds.push(i + 1);
                }
            },
            _ => (),
        }
        i += 1;
    }
    if !split_inds.contains(&i) {
        split_inds.push(i);
    }

    for i in 0..(split_inds.len()-1) {
        sequences.push(s[split_inds[i]..split_inds[i+1]].to_string());
    }

    return sequences;
}

fn split_branches(s: &String) -> Vec<String> {
    let mut options = Vec::new();

    if !(s.starts_with('(') && s.ends_with(')')) {
        //println!("ERROR! split_branches no parenthesis: {}", s);
        options.push(s.clone()); // TODO: May be a problem
        return options;
    }

    let mut level = 0;
    let mut split_bounds = vec!(0);
    let mut i = 0;
    for c in s.chars() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            '|' => {
                if level == 1 {
                    split_bounds.push(i);
                }
            }
            _ => (),
        }
        i += 1;
    }
    split_bounds.push(i - 1);

    for i in 0..(split_bounds.len()-1) {
        options.push(s[(split_bounds[i]+1)..split_bounds[i+1]].to_string());
    }

    return options;
}



