//https://cses.fi/alon/task/1074
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
 
pub fn read_lines() -> (usize,Vec<u32>) {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<u32>= Vec::with_capacity(number);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<u32>().unwrap());
    }
    if number != vect.len() {
        panic!("declared lenght and real lengths are different, panic!");
    }
    (number,vect)
}
 
fn main() {
    let (size,mut data) = read_lines();
    data.sort();
    let med:u64 = data[size / 2] as u64;
    let mut b:u64 = 0;
    for elt in data.iter() {
        if *elt as u64 > med {
            b+= *elt as u64 - med ;
        } else {
            b+=med-*elt as u64
        }
    }
    print!("{}",b);
}
