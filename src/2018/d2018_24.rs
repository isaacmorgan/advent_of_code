use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::max;

const FNAME: &str = "./input/2018/2018-24.txt";

#[derive(Debug, Clone)]
struct Group {
    id: i64,
    team: i64,
    units: i64,
    hp: i64,
    dmg: i64,
    dmg_type: String,
    init: i64,
    weak: Vec<String>,
    immune: Vec<String>,
}

pub fn main() {
    let mut min_boost = 0;
    let mut max_boost = 72;
    let mut prev_boost = 0;
    let mut boost = min_boost; //min_boost + max_boost / 2;
    let mut infection_units = 0;
    let mut immune_units = 0;
    let mut old_infection_units = 0;
    let mut old_immune_units = 0;
    loop {
        boost = boost + 1;//(min_boost + max_boost) / 2;
        let (mut immune_system, mut infection) = load();
        for g in &mut immune_system {
            g.dmg += boost;
        }
        loop {
            // Target Selection
            let mut target_map = target_selection(&mut immune_system, &infection);
            let tmp = target_selection(&mut infection, &immune_system);
            target_map.extend(tmp);
            //println!("{:?}", target_map);

            // Consolidate groups
            let mut groups = Vec::new();
            groups.extend(immune_system);
            groups.extend(infection);

            // Attack Phase
            attack_phase(&mut groups, &target_map);

            //println!("{:?}", groups);

            // Split groups again
            immune_system = Vec::new();
            infection = Vec::new();
            for g in groups {
                if g.units > 0 {
                    match g.team {
                        0 => immune_system.push(g),
                        1 => infection.push(g),
                        _ => (),
                    }
                }
            }

            if immune_system.is_empty() || infection.is_empty() {
                //println!("Immune System: {:?}", immune_system);
                //println!("Infection: {:?}", infection);
                break;
            }
            immune_units = sum_units(&immune_system);
            infection_units = sum_units(&infection);
            if immune_units == old_immune_units && infection_units == old_infection_units {
                println!("Immune System: {:?}", immune_system);
                println!("Infection: {:?}", infection);
                break;
            } else {
                old_immune_units = immune_units;
                old_infection_units = infection_units;
            }
        }
        println!("Boost: {} - {} - {}", min_boost, boost, max_boost);
        println!("Immune System Units: {:?}", sum_units(&immune_system));
        println!("Infection Units: {:?}", sum_units(&infection));
        if sum_units(&infection) == 0 {
            max_boost = boost;
            break;
        } else {
            min_boost = boost;
        }
        if boost == prev_boost {
            break;
        }
        prev_boost = boost;
    }
    println!("Boost required: {}", max_boost);
}

fn sum_units(g: &Vec<Group>) -> i64 {
    g.iter().map(|a| a.units).sum()
}

fn attack_phase(groups: &mut Vec<Group>, target_map: &HashMap<i64, i64>) {
    //println!("Attack Phase");
    groups.sort_by(|a, b| b.init.cmp(&a.init));
    let mut ind_map = HashMap::new();
    let mut cnt = 0;
    for g in &*groups {
        ind_map.insert(g.id, cnt);
        cnt += 1;
    }
    for i in 0..(&*groups).len() {
        if groups[i].units <= 0 {
            continue;
        }
        if !target_map.contains_key(&groups[i].id) {
            continue;
        }
        let j = *ind_map.get(&target_map.get(&groups[i].id).unwrap()).unwrap();
        attack(groups, i, j);
    }
}

fn get_group(groups: &mut Vec<Group>, id: i64) -> Option<&mut Group> {
    for g in groups {
        if g.id == id {
            return Some(g);
        }
    }
    return None;
}

fn target_selection(g_atk: &mut Vec<Group>, g_def: &Vec<Group>) -> HashMap<i64, i64> {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    g_atk.sort_by(|a, b| (effective_power(b)*1_000 + b.init).cmp(&(effective_power(a)*1_000 + a.init)) );
    /*for q in &*g_atk {
        println!("id: {} power: {} calc: {}", q.id, effective_power(q), effective_power(q) * 1_000 + q.init);
    }*/
    for a in g_atk {
        let mut chosen_id = -1;
        let mut max_dmg = 1;
        let mut chosen_d = g_def[0].clone();
        for d in g_def {
            if set.contains(&d.id) {
                continue
            }
            let est_dmg = calc_dmg(a, d);
            if est_dmg > max_dmg
            || (est_dmg == max_dmg && effective_power(d) > effective_power(&chosen_d))
            || (est_dmg == max_dmg && effective_power(d) == effective_power(&chosen_d) && d.init > chosen_d.init) {
                chosen_id = d.id;
                max_dmg = est_dmg;
                chosen_d = d.clone();
            }
        }
        if chosen_id > -1 {
            map.insert(a.id, chosen_id);
            set.insert(chosen_id);
        }
    }
    return map;
}

fn attack(groups: &mut Vec<Group>, ind_a: usize, ind_b: usize) {
    let dmg = calc_dmg(&groups[ind_a], &groups[ind_b]) / groups[ind_b].hp;
    //println!("Attack: {:?}\nDefend: {:?}\nDmg: {}", &groups[ind_a], &groups[ind_b], dmg);
    groups[ind_b].units -= dmg;
}

fn calc_dmg(g_atk: &Group, g_def: &Group) -> i64 {
    effective_power(g_atk) *
        if g_def.immune.contains(&g_atk.dmg_type) {
            0
        } else if g_def.weak.contains(&g_atk.dmg_type) {
            2
        } else {
            1
        }
}

fn effective_power(group: &Group) -> i64 {
    group.units * group.dmg
}

fn load() -> (Vec<Group>, Vec<Group>) {
    let contents = std::fs::read_to_string(FNAME).expect("Error reading file");
    let mut team = 0;
    let mut immune_system = Vec::new();
    let mut infection = Vec::new();
    let mut cnt = 0;
    for c in contents.lines() {
        //println!("{:?}", c);
        match c {
            "Immune System:" => team = 0,
            "Infection:" => team = 1,
            "" => (),
            _ => match team {
                0 => immune_system.push(line_to_group(c, cnt, team)),
                1 => infection.push(line_to_group(c, cnt, team)),
                _ => (),
            }
        };
        cnt += 1;
    }
    //println!("{:?}", immune_system);
    //println!("{:?}", infection);
    /*for a in &immune_system {
        for b in &infection {
            //println!("{:?} -> {:?} \n\t {}", a, b, calc_dmg(a, b))
        }
    }
    */
    return (immune_system, infection);
}

fn line_to_group(line: &str, id: i64, team: i64) -> Group {
    let re = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\((.*)\))*\s?with an attack that does (\d+) ([a-z, ]+) damage at initiative (\d+)").unwrap();
    let cap = re.captures(line).unwrap();

    let units = cap[1].parse().unwrap();
    let hp = cap[2].parse().unwrap();
    let dmg = cap[4].parse().unwrap();
    let dmg_type = cap[5].to_string();
    let init = cap[6].parse().unwrap();
    let (weak, immune) = parse_weak_immune(if cap.get(3).is_some() { &cap[3] } else { "" });
    let group = Group {
        id,
        team,
        units,
        hp,
        dmg,
        dmg_type,
        init,
        weak,
        immune,
    };
    //println!("{:?}", cap);
    //println!("{:?}", group);
    return group;
}

fn parse_weak_immune(line: &str) -> (Vec<String>, Vec<String>) {
    let re_weak = Regex::new(r"weak to ([a-z, ]+)").unwrap();
    let re_immune = Regex::new(r"immune to ([a-z, ]+)").unwrap();
    let cap_weak = re_weak.captures(line);
    let cap_immune = re_immune.captures(line);
    //println!("{:?}", cap_weak);
    //println!("{:?}", cap_immune);

    let mut weak = Vec::new();
    let mut immune = Vec::new();

    if cap_weak.is_some() {
        cap_weak.unwrap()[1].split(", ").for_each(|c| weak.push(c.to_string()));
    }
    if cap_immune.is_some() {
        cap_immune.unwrap()[1].split(", ").for_each(|c| immune.push(c.to_string()));
    }
    return (weak, immune);
}
