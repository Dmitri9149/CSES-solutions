//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;
use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn read_lines() -> (usize,usize,Vec<usize>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut first_line = iter_line
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
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for i in 0..integers {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection.push(seed);
        }
        break;
    }
    (integers, subarrays,collection)
}

fn main() {
    let (integers, subarrays,collection) = read_lines();
    let mut count:usize=0;
    let mut left:usize=0;
    let mut right:usize=0;
    let mut map:BTreeMap<usize,usize>= BTreeMap::new();
    while right < integers {
        *map
            .entry(collection[right])
            .or_insert(0)+=1;
        while map.keys().len() > subarrays {
            if *map.get(&collection[left]).unwrap() == 1 {
                map.remove(&collection[left]); 
            } else {
                *map
                .entry(collection[left])
                .or_insert(collection[left])-=1;
            }

            left +=1;
        }
        count +=right - left +1;
        right+=1;
    }
    println!("{}",count);
}

