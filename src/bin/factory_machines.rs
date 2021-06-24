use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;

pub fn read_lines() -> (usize,usize,Vec<u64>) {
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

    let mut collection:Vec<u64> = Vec::with_capacity(machines);
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for _i in 0..machines {
            seed = iter.next().unwrap().parse::<u64>().unwrap();
            collection.push(seed);
        }
        break;
    }
    (machines, products,collection)
}

fn main() {
    let (machines, products,collection) = read_lines();
    let mut low = 0;
    let mut high:u64 = 1000000000000000000;
    let mut res:u64 = 1000000000000000000;
    while low <= high {
//        println!("mid: {}  prod: {}  low: {  } high: {}   res: {}",mid,prod,low,high,res);
        let mut mid;
        let mut prod = 0;
        mid = (low + high) / 2;
        for (i,time)  in collection.iter().enumerate() {
            prod += std::cmp::min(mid as u64/time,1000000000);
        }
        if prod >= products as u64 {
            if mid < res {
                res = mid;
                high = mid -1;         
            }
        } else {
            low = mid + 1;
        } 
//        println!("mid: {}  prod: {}  low: {  } high: {}   res: {}",mid,prod,low,high,res);

    }
    println!("{}",res);
}

