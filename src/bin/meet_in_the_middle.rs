//https://cses.fi/alon/task/2428
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::collections::BTreeMap;

pub fn read_lines() -> (usize,u32,Vec<u32>,Vec<u32>) {
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
        .parse::<u32>().unwrap();

    let mut collection1:Vec<u32> = Vec::with_capacity(integers);
    let mut collection2:Vec<u32> = Vec::with_capacity(integers);
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for _i in 0..(integers/2) {
//            println!("{}",_i);
            seed = iter.next().unwrap().parse::<u32>().unwrap();
            collection1.push(seed);
        }
        for _i in (integers/2)..integers {
//            println!("new {}",_i);
            seed = iter.next().unwrap().parse::<u32>().unwrap();
            collection2.push(seed);
        }

        break;
    }
    (integers, subarrays,collection1,collection2)
}

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element)
                             .collect()
     }).collect()
}

fn summ(s: &[u32]) -> BTreeMap<u32,u32> {
    let mut tree:BTreeMap<u32,u32> = BTreeMap::new();
    for elt in (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
        .map(|(_, element)| *element).sum()
    }) {
        *tree
            .entry(elt)
            .or_insert(0)+=1;
    }
    tree
}
fn main() {
    let (total,sub_sum,set1,set2) = read_lines();
    let mut count:u32=0;
//    let mut left;
//    let mut right;
    if total == 1 && sub_sum == 1{
        println!("{}",sub_sum);
        return ()
    }
    let mut sums1:BTreeMap<u32,u32>= BTreeMap::new();
    let mut sums2:BTreeMap<u32,u32>= BTreeMap::new();
    sums1 = summ(&set1);
//    println!("sum 1 {:?}",sums1);
    sums2 = summ(&set2);
//    sums2.sort();

    let mut state; 
    let mut left_iter;
    left_iter = sums1.iter();
    loop {
        match left_iter.next() {
            None => break,
            Some(x) => state = x,
        }
        if state.0 > &sub_sum {
            break;
        }
        match sums2.get(&(sub_sum-state.0)) {
            None => continue,
            Some(x) => count+=state.1*x,
        }
    }
//    println!("{:?}    {:?}",&sums1,&sums2);
    println!("{}",count);
}

