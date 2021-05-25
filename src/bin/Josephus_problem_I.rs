use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::BTreeMap;

fn main() {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<u64>().unwrap();
    println!("number: {}",number);
}
