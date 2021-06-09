//https://cses.fi/alon/task/2428
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
    let integers = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let subarrays = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    let mut collection:Vec<usize> = Vec::with_capacity(integers);
    let mut seed;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for _i in 0..integers {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection.push(seed);
        }
        break;
    }
    (integers, subarrays,collection)
}
pub fn is_possible(value:&usize, collection:&Vec<usize>,k:&usize) -> bool {
    let mut count = 0;
    let mut sum = 0;
    for elt in collection.iter() {
        if elt > value {
            return false
        }
        sum +=elt;

        if sum > *value {
            count+=1;
            sum = *elt;
        }
    }
    count+=1;

    if count <= *k {
        true
    } else {
        false
    }
}
pub fn min_max(collection:&Vec<usize>,k:&usize) -> usize {
    let mut start = collection.iter().max().unwrap().to_owned();
    let mut end = 0;
    let mut res=0;
    let mut value;
    for elt in collection.iter() {
        end+=elt;
    }
    while start <= end {
        value = (start+end) / 2 ;
        if is_possible(&value,&collection,&k) {
            res = value;
            end = value -1;
        } else {
            start = value + 1;
        }

    }
    return res
}
fn main() {
    let (integers, subarrays,collection) = read_lines();
    println!("{}",min_max(&collection,&subarrays));
}

