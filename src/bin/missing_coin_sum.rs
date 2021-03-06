/*
You have n coins with positive integer values.
What is the smallest sum you cannot create using a subset of the coins?

Input

The first input line has an integer n: the number of coins.

The second line has n integers x1,x2,…,xn: the value of each coin.

Output

Print one integer: the smallest coin sum.

Constraints
1≤n≤2⋅105
1≤xi≤109
*/

use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> Vec<u64> {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<u64>().unwrap();
    let mut vect:Vec<u64>= Vec::new();
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<u64>().unwrap());
    }
    vect
}

fn main() {
    let mut vect = read_lines();
    vect.sort();
    let mut current_sum =0;
    for elt in vect.iter() {
        if elt > &(current_sum +1) {
            break;
        } else {current_sum +=elt;}
    }
    println!("{}",current_sum+1);
}
