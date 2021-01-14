use itertools::enumerate;
use std::collections::HashSet;

mod intcode;

const FNAME: &str = "./input/2019/2019-23.txt";
pub fn main() {
    let mut comps = Vec::new();
    for i in 0..50 {
        let mut comp = intcode::setup_new_computer(FNAME);
        comp.input.push(i);
        comps.push(comp);
    }
    let mut NAT = (0, 0);
    let mut set = HashSet::new();
    
    'outer: loop {
        let mut idle = true;
        for i in 0..comps.len() {
            let mut v = Vec::new();
            {
                let c = &mut comps[i];
                c.input.push(-1);
                while intcode::step(c) {};
                //println!("comp {}: {:?}", i, c.output);
                while !c.output.is_empty() {
                    let j = c.output.remove(0);
                    let x = c.output.remove(0);
                    let y = c.output.remove(0);
                    v.push((j, x, y));
                    idle = false;
                }
            }
            for (j, x, y) in v {
                if j == 255 {
                    //println!("End Packet: a:{} x:{} y:{}", j, x, y);
                    NAT = (x, y);
                    break;
                }
                comps[j as usize].input.push(x);
                comps[j as usize].input.push(y);
            }
        }
        //println!("Iter Done");

        if idle {
            // NAT send packet to address 0
            comps[0].input.push(NAT.0);
            comps[0].input.push(NAT.1);
            if set.contains(&NAT.1) {
                println!("{:?}", set);
                println!("Duplicate NAT Value: {}", NAT.1);
                break 'outer;
            }
            set.insert(NAT.1);
        }
    }
}