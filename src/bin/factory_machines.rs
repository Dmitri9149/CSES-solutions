use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;

pub fn read_lines() -> (usize,usize,Vec<u32>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let machines = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let products = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    let mut collection:Vec<u32> = Vec::with_capacity(machines);
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for _i in 0..machines {
            seed = iter.next().unwrap().parse::<u32>().unwrap();
            collection.push(seed);
        }
        break;
    }
    (machines, products,collection)
}

fn main() {
    let (machines, products,collection) = read_lines();
    println!("{:?} {:?} {:?}",machines, products,collection);
}

