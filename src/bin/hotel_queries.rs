//https://cses.fi/alon/task/1628/
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::BTreeMap;
use std::collections::HashMap;


pub fn read_lines() -> (usize,usize,Vec<usize>,Vec<usize>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let hotels = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let rooms = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize >().unwrap();

    let mut collection1:Vec<usize> = Vec::with_capacity(hotels);
    let mut collection2:Vec<usize > = Vec::with_capacity(rooms);
    let mut seed;
    let mut line;
    let mut input;
    line = iter_line.next().unwrap();
    input = line.expect("Failed to last line");
    iter = input.split_whitespace();
    for _i in 0..(hotels) {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection1.push(seed);
    }
    line = iter_line.next().unwrap();
    input = line.expect("Failed to last line");
    iter = input.split_whitespace();
    for _i in 0..(rooms) {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection2.push(seed);
    }
    (hotels,rooms,collection1,collection2)
}



fn main() {
    let (hotels,rooms,set1,set2) = read_lines();
    println!("{:?}  {:?}",set1,set2);
}

