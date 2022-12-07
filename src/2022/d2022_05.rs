use itertools::Itertools;
use regex::Regex;
use crate::tools;
static FNAME: &str = "./input/2022/2022-05-01.txt";

pub fn main() {
    let input = tools::load(FNAME);
    part1(&input);
    part2(&input);
}

fn part1(lines: &Vec<String>) {
    let mut model = 9000;
    let (mut stacks,commands) = parse(lines);
    run_commands(&mut stacks, &commands, model);
    println!("Part 1 Message: {}", top_crate_message(&stacks));
}

fn part2(lines: &Vec<String>) {
    let mut model = 9001;
    let (mut stacks,commands) = parse(lines);
    run_commands(&mut stacks, &commands, model);
    println!("Part 2 Message: {}", top_crate_message(&stacks));
}

fn top_crate_message(stack: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for st in stack {
        s.push(*st.last().unwrap());
    }
    return s;
}

fn run_commands(stack: &mut Vec<Vec<char>>, commands: &Vec<&String>, model: i32) {
    let mut hold: Vec<char> = Vec::new();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for cmd in commands {
        let cap = re.captures(cmd).unwrap();
        let (n, s, d): (usize, usize, usize) = cap.iter().skip(1).map(
            |x| x.unwrap().as_str().parse().unwrap()).collect_tuple().unwrap();
        match model {
            9000 => {
                for _ in 0..n {
                    hold.push(stack[s-1].pop().unwrap());
                    stack[d-1].push(hold.pop().unwrap());
                }
            },
            9001=> {
                for _ in 0..n {
                    hold.push(stack[s-1].pop().unwrap());
                }
                for _ in 0..n {
                    stack[d-1].push(hold.pop().unwrap());
                }
            },
            _ => (),
        }
        //dbg!(&cmd);
        //dbg!(&stack);
    }

}

fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<&String>) {
    let idx_blank = lines.iter().position(|x| x.is_empty()).unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines_stack = lines.iter().take(idx_blank).rev();
    let n = lines_stack.next().unwrap().split("   ").count();

    // Create stacks
    for _ in 0..n {
        stacks.push(Vec::new());
    }

    // Populate stacks
    lines_stack.for_each(|x|
        x.chars().skip(1).step_by(4).enumerate().for_each(|(i, c)|
            if c != ' ' {
                stacks[i].push(c)
            }
        )
    );

    return (stacks, lines.iter().skip(idx_blank + 1).collect())
}
