use std::collections::HashMap;

const FNAME: &str = "./input/2019/2019-14.txt";

pub fn main() {
    let map = load();
    println!("Map: {:?}", map);

    let mut ore_cnt = 0;
    let mut ing = HashMap::new();
    let mut extra = HashMap::new();
    ing.insert("FUEL".to_string(), 1);
    loop {
        if ing.is_empty() {
            break;
        }
        substitute(&mut ing, &mut extra, &map);
        ore_cnt += ing.remove("ORE").unwrap_or_default();
    }
    println!("Ore count: {}", ore_cnt);

    // Guess and check for how many fuel results in < 1_000_000_000_000 ore
    let mut n_max = 1_000_000_000;
    let mut n_min = 0;
    let mut n = 0;
    while n_max != n_min {
        n = (n_max + n_min)/2;
        let mut ore_cnt = 0;
        let mut ing = HashMap::new();
        let mut extra = HashMap::new();
        ing.insert("FUEL".to_string(), n);
        loop {
            if ing.is_empty() {
                break;
            }
            substitute(&mut ing, &mut extra, &map);
            ore_cnt += ing.remove("ORE").unwrap_or_default();
        }
        if ore_cnt > 1_000_000_000_000 {
            if n_max == n {
                break;
            }
            n_max = n;
        } else if ore_cnt < 1_000_000_000_000 {
            if n_min == n {
                break;
            }
            n_min = n;
        } else {
            break;
        }
    }
    println!("{} ORE makes {} FUEL", ore_cnt, n);
}

fn substitute(ing: &mut HashMap<String, i64>, extra: &mut HashMap<String, i64>, map: &HashMap<String, Vec<(i64, i64, String)>>) {
    use_extra(ing, extra, map);

    let ik = ing.keys().next().unwrap().clone();
    let iv = ing.remove(&ik).unwrap();

    let recipe = map.get(&ik).unwrap();
    let m = if iv.rem_euclid(recipe.first().unwrap().0) == 0 { iv/recipe.first().unwrap().0 } else { iv/recipe.first().unwrap().0 + 1 };
    for r in recipe {
        ing.insert(r.2.clone(), if ing.contains_key(&r.2) { ing.get(&r.2).unwrap() + m*r.1 } else { m*r.1 });
    }
    let ev = recipe.first().unwrap().0 * m - iv;
    if ev > 0 {
        extra.insert(ik.clone(), ev);
    }
}

fn use_extra(ing: &mut HashMap<String, i64>, extra: &mut HashMap<String, i64>, map: &HashMap<String, Vec<(i64, i64, String)>>) {
    let mut both = Vec::new();
    for (ek, ev) in &*extra {
        if ing.contains_key(ek) {
            both.push(ek.clone());
        }
    }
    for k in &both {
        let mut ev = extra.remove(k).unwrap();
        let mut iv = ing.remove(k).unwrap();
        if iv > ev {
            iv -= ev;
            ing.insert(k.clone(), iv);
        } else if iv < ev {
            ev -= iv;
            extra.insert(k.clone(), ev);
        }
    }
}

fn count_inputs(name: &String, num: i64, map: &HashMap<String, Vec<(i64, i64, String)>>) -> (HashMap<String, i64>, HashMap<String, i64>) {
    let mut out = HashMap::new();
    let mut extra = HashMap::new();
    if !map.contains_key(name) {
        return (out, extra);
    }
    let recipe = map.get(name).unwrap();
    let m = if num.rem_euclid(recipe.first().unwrap().0) == 0 { num/recipe.first().unwrap().0 } else { num/recipe.first().unwrap().0 + 1 };
    for r in recipe {
        out.insert(r.2.clone(), if out.contains_key(&r.2) { out.get(&r.2).unwrap() + m*r.1 } else { m*r.1 });
    }
    let e = recipe.first().unwrap().0 * m - num;
    if e > 0 {
        extra.insert(name.clone(), e);
    }
    return (out, extra);
}

fn load() -> HashMap<String, Vec<(i64, i64, String)>> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    let mut out = HashMap::new();
    let re = regex::Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split("=>");
        let mut inp: Vec<(i64, i64, String)> = Vec::new();
        let inps = re.captures_iter(parts.next().unwrap());
        let outps = re.captures(parts.next().unwrap());
        let caps = outps.iter().next().unwrap();
        let n_out = caps[1].parse::<i64>().unwrap();
        let s_out = caps[2].to_string();
        for cap in inps {
            inp.push((n_out, cap[1].parse::<i64>().unwrap(), cap[2].to_string()));
        }
        out.insert(s_out, inp);
    }
    return out;
}
