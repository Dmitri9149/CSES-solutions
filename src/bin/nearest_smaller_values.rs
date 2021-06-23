use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::convert::TryInto;
 
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
    let mut stack:Vec<(u32,u32)> = Vec::with_capacity(size);
    for i in 0..size {
        while stack.len() > 0 && stack[stack.len()-1].1 >= data[i] {
            stack.pop();
        }
        if stack.len() == 0 {
            print!("{} ",0);
        } else {
            print!("{} ",stack[stack.len()-1].0+1);
        }
        stack.push((i.try_into().unwrap(),data[i]));
    }
}
