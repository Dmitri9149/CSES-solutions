//https://cses.fi/alon/task/1744/
use std::io::{BufRead};
use std::io;
use std::cmp;

pub fn read_lines() -> (u32,u32) {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let aa = fst_line_iter
        .next()
        .unwrap()
        .parse::<u32>().unwrap();
    let bb = fst_line_iter
        .next()
        .unwrap()
        .parse::<u32>().unwrap();

    (aa, bb)
}
fn main() {
    let (aa, bb) = read_lines();
    println!("{}", 100);
}
