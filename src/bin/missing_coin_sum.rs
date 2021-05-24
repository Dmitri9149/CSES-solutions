use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;


fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    line.pop();
    let number = line.parse::<u64>().unwrap();

    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    line.pop();
    
    let mut split = line.split_whitespace();
    let mut tree:BTreeMap<u64,u64> = BTreeMap::new();
    for elt in split {
        *tree
            .entry(elt.parse::<u64>().unwrap())
            .or_insert(0)+=1;
    }
    println!("tree {:?}",tree);
}
