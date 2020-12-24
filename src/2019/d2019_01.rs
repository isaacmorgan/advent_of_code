use std::cmp::max;

const FNAME: &str = "./input/2019/2019-01.txt";

pub fn main() {
    let data = load();
    let s = data.iter().map(|x| calc_fuel(*x)).sum::<i32>();
    println!("Fuel required: {}", s);
    let s = data.iter().map(|x| calc_fuel_recursive(*x)).sum::<i32>();
    println!("Fuel required: {}", s);
}

fn load() -> Vec<i32> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    let mut data = Vec::new();
    for x in input.split("\n") {
        if x.is_empty() { continue }
        data.push(x.parse().unwrap());
    }
    return data;
}

fn calc_fuel(mass: i32) -> i32 {
    max(0, mass/3 - 2)
}

fn calc_fuel_recursive(mass: i32) -> i32 {
    if mass == 0 {
        0
    } else {
        let module_fuel = calc_fuel(mass);
        max(0, module_fuel + calc_fuel_recursive(module_fuel))
    }
}