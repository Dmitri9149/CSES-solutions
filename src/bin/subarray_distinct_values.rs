// https://cses.fi/alon/task/1164
//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> (usize,usize,Vec<usize>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let integers = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let subarrays = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line")
        .parse::<usize>().unwrap();

    let mut collection:Vec<usize> = Vec::with_capacity(integers);
    let mut index = 0;
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection.push(seed);
        index+=1;
        if index  == integers {
            break;
        }
    }
    (integers, subarrays,collection)
}

fn main() {
    let (integers, subarrays,collection) = read_lines();
}

