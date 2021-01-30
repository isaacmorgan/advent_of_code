use crate::tools;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static FNAME: &str = "./input/2020/2020-21.txt";

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let list = load();
    // for l in &list {
    //     println!("{:?}", l);
    // }
    let mut map = map_allergens(&list);
    // println!();
    // println!("{:?}", map);
    map_fix_singles(&mut map);
    println!("{:?}", map);
    
    let final_ing = map.values().into_iter().flatten().map(|x| x.clone()).collect_vec();
    let mut cnt = 0;
    for (ingredients, _) in &list {
        cnt += ingredients.iter().filter(|x| !final_ing.contains(*x)).count();
    }
    
    println!("Unknown ingredient count: {}", cnt);
}

fn part2() {
    let list = load();
    let mut map = map_allergens(&list);
    map_fix_singles(&mut map);
    
    println!("{:?}", map);
    
    let mut allergens = map.keys().collect_vec();
    allergens.sort();
    let mut out = String::new();
    for al in allergens {
        if !out.is_empty() {
            out.extend(",".chars());
        }
        out.extend(map.get(al).unwrap().iter().next().unwrap().chars());
    }
    println!("Dangerous Ingredients:\n{}", out);
    

}

fn map_fix_singles(map: &mut HashMap<String, HashSet<String>>) {
    let keys = map.keys().into_iter().map(|x| x.clone()).collect_vec();
    let mut change;
    loop {
        let mut single_ing = Vec::new();
        change = false;
        
        for k in &keys {
            let &val = &map.get(k).unwrap();
            if val.len() == 1 {
                single_ing.push(val.iter().next().unwrap().clone());
            }
        }
        
        for ing in &single_ing {
            for k in &keys {
                let val = map.get_mut(k).unwrap();
                if val.len() != 1 && val.contains(ing) {
                    val.remove(ing);
                    change = true;
                }
            }
        }
        
        if change == false {
            break;
        }
    }
    
}

fn map_allergens(list: &Vec<(Vec<String>, Vec<String>)>) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for (ingredients, allergens) in list {
        for al in allergens {
            if map.contains_key(al) {
                let almap = map.get_mut(al).unwrap();
                almap.retain(|ing| {
                    ingredients.contains(ing)
                });
            } else {
                let mut almap = HashSet::new();
                almap.extend(ingredients.clone());
                map.insert(al.clone(), almap);
            }
        }
    }
    return map;
}

fn load() -> Vec<(Vec<String>, Vec<String>)> {
    let input = tools::load(FNAME);
    let mut out = Vec::new();
    for line in &input {
        let mut parts = line.split(" (contains ");
        let ingredients = parts.next().unwrap().split(" ").map(|x| x.to_string()).collect_vec();
        let allergens = parts.next().unwrap().trim_end_matches(")").split(", ").map(|x| x.to_string()).collect_vec();
        out.push((ingredients, allergens));
    }
    return out;
}