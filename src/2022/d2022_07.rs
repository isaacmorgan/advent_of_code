use std::collections::HashMap;
use std::thread::current;
use itertools::Itertools;
use crate::tools;

static FNAME: &str = "./input/2022/2022-07-01.txt";

pub fn main() {
    let input = tools::load(FNAME);
    let map = part1(&input);
    part2(&map);
}

#[derive(Debug, PartialEq)]
enum RecordType {
    File,
    Folder,
}

#[derive(Debug)]
struct Record {
    name: String,
    rtype: RecordType,
    children: Option<Vec<String>>,
    size: i32,
}

fn part1(lines: &Vec<String>) -> HashMap<String, Record> {
    let mut map = parse(lines);
    let total_size = calc_size(&mut map, "/");
    let sum_sm: i32 = map.values().filter(|x| (x.rtype == RecordType::Folder) && (x.size <= 100000)).map(|x| x.size).sum();
    println!("Total size under 100,000: {sum_sm}");
    return map;
}

fn part2(map: &HashMap<String, Record>) {
    let disk_space = 70_000_000;
    let required_space = 30_000_000;
    let used_space = map.get("/").unwrap().size;
    let needed_space = required_space - (disk_space - used_space);
    let size = map.values()
        .filter(|x| x.rtype == RecordType::Folder)
        .map(|x| x.size)
        .filter(|x| x > &needed_space)
        .min().unwrap();
    println!("Smallest folder size: {size}");
}

fn parse(lines: &Vec<String>) -> HashMap<String, Record> {
    let mut path: Vec<String> = Vec::new();
    let mut current_dir: &mut Record;
    let mut map: HashMap<String, Record> = HashMap::new();
    map.insert("/".to_string(), Record{
        name: "/".to_string(),
        rtype: RecordType::Folder,
        children: None,
        size: 0
    });
    for ln in lines {
        let split = ln.split_once(' ').unwrap();
        match split {
            ("$", "ls") => {
                // Do nothing. The following is a file/directory listing.
            }
            ("$", "cd ..") => {
                path.pop();
                current_dir = map.get_mut(&path.join("")).unwrap();
            }
            ("$", "cd /") => {
                path.clear();
                path.push("/".to_string());
            }
            ("$", cd_rem) => {
                if let ("cd", dname) = cd_rem.split_once(' ').unwrap() {
                    // Create new folder
                    path.push(format!("{}/", dname));
                    current_dir = map.get_mut(&path.join("")).unwrap();
                } else {
                    panic!("Expected string format `$ cd fname`, found: {ln}");
                }
            }
            ("dir", dname) => {
                // Create new folder
                let record = Record {
                    name: dname.to_string(),
                    rtype: RecordType::Folder,
                    children: None,
                    size: 0
                };
                map.insert(format!("{}{}/",&path.join(""), dname), record);
                // Mark as child in current dir
                current_dir = map.get_mut(&path.join("")).unwrap();
                current_dir.children.get_or_insert(Vec::new()).push(format!("{}{}/",&path.join(""), dname).to_string());
            }
            (size, fname) => {
                // Create new file
                let mut record = Record {
                    name: fname.to_string(),
                    rtype: RecordType::File,
                    children: None,
                    size: size.parse::<i32>().unwrap()
                };
                map.insert(format!("{}{}", &path.join(""), fname), record);
                // Mark as child in current dir
                current_dir = map.get_mut(&path.join("")).unwrap();
                current_dir.children.get_or_insert(Vec::new()).push(format!("{}{}",&path.join(""), fname).to_string());
            }
            _ => { panic!("No pattern match found for line: {ln}"); }
        }
    }
    return map;
}

fn calc_size(map: &mut HashMap<String, Record>, name: &str) -> i32 {
    let record = map.get(name).unwrap();
    let mut size = record.size;
    let mut children_copy;
    if let Some(children) = &record.children {
        children_copy = children.clone();
        for c in children_copy {
            size = size + calc_size(map, c.as_str());
        }
    }
    let record = map.get_mut(name).unwrap();
    record.size = size;
    return size;
}