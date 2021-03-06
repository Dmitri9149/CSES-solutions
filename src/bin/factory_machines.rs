use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> (usize,usize,Vec<usize>) {
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

    let mut collection:Vec<usize> = Vec::with_capacity(machines);
    let mut seed;
    for line in iter_line {
        let input = line.expect("Failed to read last line");
        iter = input.split_whitespace();
        for _i in 0..machines {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection.push(seed);
        }
        break;
    }
    (machines, products,collection)
}

fn main() {
    let (_machines, products,collection) = read_lines();
    let mut low = 0;
    let mut high:usize = 1000000000000000000;
    let mut res:usize = 1000000000000000000;
    while low <= high {
        let mid;
        let mut prod = 0;
        mid = (low + high) / 2;
        for (_i,time)  in collection.iter().enumerate() {
            prod += std::cmp::min(mid/time,1000000000);
        }
        if prod >= products {
            if mid < res {
                res = mid;
            }
            high = mid -1;
        } else {
            low = mid + 1;
        } 
    }
    println!("{}",res);
}

