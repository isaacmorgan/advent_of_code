const N: usize = 6;
const MIN: i32 = 147981;
const MAX: i32 = 691423;

pub fn main() {
    let n = count_pass([1, 4, 7, 9, 8, 1], 0);
    println!("Password Count: {}", n);
    let pat = [1,1,2,2,3,3];
    println!("strict double: {:?} - {}", pat, check_doubles_strict(&pat));
    let pat = [1,2,3,4,4,4];
    println!("strict double: {:?} - {}", pat, check_doubles_strict(&pat));
    let pat = [1,1,1,1,2,2];
    println!("strict double: {:?} - {}", pat, check_doubles_strict(&pat));
}

fn count_pass(mut pass: [i32; N], ind: usize) -> i32 {
    let mut s = 0;
    if ind >= N {
        if valid_pass(&pass) {
            s = 1;
        }
    } else if ind > 0 {
        for i in pass[ind-1]..=9 {
            pass[ind] = i;
            s += count_pass(pass, ind + 1);
        }
    } else {
        for i in 0..=9 {
            pass[ind] = i;
            s += count_pass(pass, ind + 1);
        }
    }
    return s;
}

fn valid_pass(pass: &[i32; N]) -> bool {
    let pint = pass_to_int(&pass);
    pint >= MIN
    && pint <= MAX
    && check_doubles_strict(&pass)
    && check_increase(&pass)
}

fn check_increase(pass: &[i32; N]) -> bool {
    for i in 0..N-1 {
        if pass[i] > pass[i+1] {
            return false;
        }
    }
    return true;
}

fn check_doubles(pass: &[i32; N]) -> bool {
    for i in 0..N-1 {
        if pass[i] == pass[i+1] {
            return true;
        }
    }
    return false;
}

fn check_doubles_strict(pass: &[i32; N]) -> bool {
    let mut run_cnt = 0;
    let mut val = pass[0];
    for i in 0..N {
        if pass[i] == val {
            run_cnt += 1;
        } else {
            if run_cnt == 2 {
                return true;
            }
            val = pass[i];
            run_cnt = 1;
        }
    }
    return run_cnt == 2;
}

fn pass_to_int(pass: &[i32; N]) -> i32 {
    pass[0]*100_000 + pass[1]*10_000 + pass[2]*1_000 + pass[3]*100 + pass[4]*10 + pass[5]
}