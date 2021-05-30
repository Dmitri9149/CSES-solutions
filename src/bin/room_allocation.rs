// https://cses.fi/alon/task/1164
//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;
use std::convert::TryInto;

pub fn read_lines() -> Vec<((u32,u32),u32)> {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<u32>().unwrap();
    let mut data:Vec<((u32,u32),u32)> = Vec::with_capacity(number.try_into().unwrap());
    let mut seed:(u32,u32)=(0,0);
    let mut index = 0;
    for line in iter_line {
        let input = line.expect("Failed to read line");
        iter = input.split_whitespace();
        seed.0 = iter.next().unwrap().parse::<u32>().unwrap();
        seed.1 = iter.next().unwrap().parse::<u32>().unwrap();
        data.push(((seed.0,seed.1),index));
        index+=1;
        if index  == number {
            break;
        }
    }
    data
}

fn main() {
    let mut data = read_lines();
    data.sort_by(|a, b| a.0.0.partial_cmp(&b.0.0).unwrap());
    ()
}

