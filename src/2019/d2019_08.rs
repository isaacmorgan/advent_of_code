use itertools::Itertools;
use regex::internal::Input;

const FNAME: &str = "./input/2019/2019-08.txt";
const W: usize = 25;
const H: usize = 6;

pub fn main() {
    let input = load();
    println!("{:?}", input);

    let layer_count_zero = count_n(&input, &'0');
    let imin = get_min_index(&layer_count_zero);

    let layer_count_one = count_n(&input, &'1');
    let layer_count_two = count_n(&input, &'2');

    println!("Layer {} has min #0s: {}, #1*#2s: {}", imin, layer_count_zero[imin], layer_count_one[imin]*layer_count_two[imin]);

    let img = decode_image(&input);

    print_img(&img);
}

fn print_img(img: &Vec<char>) {
    for h in 0..H {
        for w in 0..W {
            print!("{}", img[h*W + w]);
        }
        println!();
    }
}

fn decode_image(img: &Vec<char>) -> Vec<char> {
    let mut dmap = vec![false; W*H];
    let mut out = vec!['2'; W*H];
    let mut layer = 0;
    loop {
        for h in 0..H {
            for w in 0..W {
                let n = h*W + w;
                let N = layer*W*H + n;
                if dmap[n] {
                    continue;
                }
                match img[N] {
                    '0' => {
                        out[n] = ' ';
                        dmap[n] = true;
                    },
                    '1' => {
                        out[n] = '*';
                        dmap[n] = true;
                    },
                    _ => (),
                }
            }
        }
        layer += 1;
        if layer*W*H >= img.len() {
            break;
        }
    }
    return out;
}

fn get_min_index(layer_count: &Vec<i32>) -> usize {
    let mut vmin = i32::max_value();
    let mut imin = 0;
    let mut cnt = 0;
    for l in layer_count {
        if l < &vmin {
            vmin = *l;
            imin = cnt;
        }
        cnt += 1;
    }
    return imin as usize;
}

fn count_n(img: &Vec<char>, c: &char) -> Vec<i32> {
    let mut layer = 0;
    let mut layer_cnt = Vec::new();
    let n = W*H;
    loop {
        let mut cnt = 0;
        for i in 0..n {
            if &img[(layer*n + i) as usize] == c {
                cnt += 1;
            }
        }
        layer_cnt.push(cnt);
        layer += 1;
        if layer*n >= img.len() {
            break;
        }
    }
    return layer_cnt
}

fn load() -> Vec<char> {
    let input = std::fs::read_to_string(FNAME).unwrap();
    input.trim().chars().collect_vec()
}
