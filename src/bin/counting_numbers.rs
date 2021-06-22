//https://cses.fi/alon/task/1744/
use std::io::{BufRead};
use std::io;
use std::cmp;

pub fn read_lines() -> (usize,usize) {
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
        .parse::<usize>().unwrap();
    let bb = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    (aa, bb)
}
fn main() {
    let (aa, bb) = read_lines();
    println!("{}", 100);
}
