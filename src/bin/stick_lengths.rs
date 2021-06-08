//https://cses.fi/alon/task/1074
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> Vec<i64> {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<i64>= Vec::with_capacity(number+1);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    vect.push(0);
    for elt in iter {
        vect.push(elt.parse::<i64>().unwrap());
    }
    vect
}

fn main() {
    let coins = read_lines();
    print!("some output");
}
