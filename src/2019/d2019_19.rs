mod intcode;

const FNAME: &str = "./input/2019/2019-19.txt";
pub fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program = program.clone();
    intcode::reboot_computer(&mut comp);

    //print_map(&mut comp, 1030, 1665, 110);
    //print_map(&mut comp, 0, 0, 100);

    // Find center slope of tractor beam
    let mut dx = 0;
    let y = 50;
    for x in 0..100 {
        intcode::reboot_computer(&mut comp);
        comp.input.push(x);
        comp.input.push(y);
        while intcode::step(&mut comp) {};
        if comp.output[0] == 1 {
            dx += x;
            break;
        }
    }
    for x in (0..100).rev() {
        intcode::reboot_computer(&mut comp);
        comp.input.push(x);
        comp.input.push(y);
        while intcode::step(&mut comp) {};
        if comp.output[0] == 1 {
            dx += x;
            break;
        }
    }
    dx = dx/2;
    let dy = y;
    println!("dx {} dy {}", dx, dy);

    // Iterative search doesn't work lol great
    // So just search along y
    let n = 100;
    let mut ymin = i64::max_value();
    for y in 10..10000 { // Skip the first few because the beam is spotty
        let x = y*dx/dy;
        println!("X: {} Y: {} N: {}", x, y, n);
        if does_square_fit(&mut comp, n, x, y) {
            ymin = y;
            break;
        }
    }

    let (x, y) = get_square_coord(&mut comp, n, ymin*dx/dy, ymin);
    println!("X: {} Y: {} Value: {}", x, y, x*10000 + y);
    /*
    // Iteratively search for location that fits the requirement
    // - Select large initial value
    let n = 100;
    let mut x = dx * 1000;
    let mut y = dy * 1000;

    let mut ymin = 0;
    let mut ymax = y;
     loop {
         x = y*dx/dy;
         println!("X: {} Y: {} min: {} max: {}", x, y, ymin, ymax);
         if does_square_fit(&mut comp, n, x, y) {
            ymax = y;
         } else {
             ymin = y;
         }
         y = (ymin + ymax ) / 2;
         if y == ymax || y == ymin {
             break;
         }
     }
    println!("Y: {}", ymax);

    let (x, y) = get_square_coord(&mut comp, n, ymax*dx/dy, ymax);
    println!("X: {} Y: {} Value: {}", x, y, x*10000 + y);*/
    // - If count is successful set max, if not set min
    // - Set next value to max+min/2, break if equal to max.
}

fn print_map (comp: &mut intcode::Computer, x0: i64, y0: i64, n: i64) {
    println!("x0: {}", x0);
    for y in y0..(y0+n) {
        print!("{}", y);
        for x in x0..(x0+n) {
            intcode::reboot_computer(comp);
            comp.input.push(x);
            comp.input.push(y);
            while intcode::step(comp) {};
            if comp.output.remove(0) == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_square_coord(comp: &mut intcode::Computer, n: i64, x:i64, y:i64) -> (i64, i64) {
    let mut x = x;
    let mut y = y;
    let mut cnt = 0;

    // Find furthest right x
    let mut found_positive = false;
    loop {
        intcode::reboot_computer(comp);
        comp.input.push(x);
        comp.input.push(y);
        while intcode::step(comp) {}
        if comp.output.remove(0) == 0 {
            if found_positive { break; }
        } else {
            found_positive = true;
        }
        x += 1;
    }

    // We just saw the top right point
    // Now check the bottom left point
    // Then check to the left for any better x fits
    x = x - n;
    y = y + n - 1;
    loop {
        intcode::reboot_computer(comp);
        comp.input.push(x);
        comp.input.push(y);
        while intcode::step(comp) {}
        if comp.output.remove(0) == 0 {
            x += 1;
            break;
        }
        x -= 1;
    }

    // If we made it here the square fits
    y -= n - 1;
    return (x, y);
}

fn does_square_fit(comp: &mut intcode::Computer, n: i64, x: i64, y: i64) -> bool {
    let mut x = x;
    let mut y = y;
    let mut cnt = 0;

    // Find furthest right x
    let mut found_positive = false;
    loop {
        intcode::reboot_computer(comp);
        comp.input.push(x);
        comp.input.push(y);
        while intcode::step(comp) {}
        if comp.output.remove(0) == 0 {
            if found_positive { break; }
        } else {
            found_positive = true;
        }
        x += 1;
    }

    // We just saw the top right point
    // Now check the bottom left point
    x = x - n;
    y = y + n - 1;
    intcode::reboot_computer(comp);
    comp.input.push(x);
    comp.input.push(y);
    while intcode::step(comp) {}
    if comp.output.remove(0) == 0 {
        return false;
    }

    // If we made it here the square fits
    return true;
}

fn part_1() {
    let mut comp = intcode::new_computer();
    let program = intcode::load_program(FNAME);
    comp.program = program.clone();
    intcode::reboot_computer(&mut comp);

    let mut cnt = 0;
    for x in 0..50 {
        for y in 0..50 {
            intcode::reboot_computer(&mut comp);
            comp.input.push(x);
            comp.input.push(y);
            while intcode::step(&mut comp) {};
            cnt += comp.output.remove(0);
        }
    }
    println!("Affected points: {}", cnt);
}

