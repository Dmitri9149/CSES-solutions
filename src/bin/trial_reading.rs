use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> Vec<(u32,u32)> {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line)
        .unwrap();
// remove the trailing '\n'
    line.pop();
    let mut number = line.parse::<u32>().unwrap();
    let stdin = io::stdin();
    let mut vect:Vec<(u32,u32)> = vec![];
    let mut iter:SplitWhitespace;
    loop {
        // Iterate over all lines that will be inputted
        for line in stdin.lock().lines() {
            let input = line.expect("Failed to read line");
            iter = input.split_whitespace();
            let mut seed:(u32,u32)=(0,0);
            seed.0 = iter.next().unwrap().parse::<u32>().unwrap();
            seed.1 = iter.next().unwrap().parse::<u32>().unwrap();
            vect.push(seed);
            number-=1;
            if number ==0 {
                return vect;
            }
        }
    }   
}

fn main() {
    read_lines();
}



