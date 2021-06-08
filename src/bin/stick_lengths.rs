//https://cses.fi/alon/task/1074
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> (usize,Vec<usize>) {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<usize>= Vec::with_capacity(number);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<usize>().unwrap());
    }
    if number != vect.len() {
        panic!("declared lenght and real lengths are different, panic!");
    }
    (number,vect)
}

fn main() {
    let data = read_lines();
    print!("data : {:?}", data);
}
