use itertools::Itertools;

pub fn load(fname: &str) -> Vec<String> {
    let input = std::fs::read_to_string(fname).unwrap();
    input.lines().map(|x| x.to_string()).collect_vec()
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}