//https://cses.fi/alon/task/1074
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
 
pub fn read_lines() -> (usize,Vec<u64>) {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<u64>= Vec::with_capacity(number);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<u64>().unwrap());
    }
    if number != vect.len() {
        panic!("declared lenght and real lengths are different, panic!");
    }
    (number,vect)
}
 
fn main() {
    let (size,mut data) = read_lines();
    data.sort();
    let med = data[size / 2];
    let mut b:i64 = 0;
    for elt in data {
        b+= (elt as u64 - med as u64).abs()
    }
    print!("{}",b);
}
