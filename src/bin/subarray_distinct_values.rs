//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;
use std::collections::BTreeSet;

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element)
                             .collect()
     }).collect()
}

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
    println!("integers {}", &integers);
    let subarrays = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    println!("subarrays {}", &subarrays);

/*
    let integers = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let subarrays = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line")
        .parse::<usize>().unwrap();
*/
    let mut collection:Vec<usize> = Vec::with_capacity(integers);
    let mut index = 0;
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for i in 0..integers {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            println!("index {} and seed {}", &index, &seed);
            collection.push(seed);
        }
        break;
    }
    (integers, subarrays,collection)
}

fn main() {
    let (integers, subarrays,collection) = read_lines();
    let mut check:BTreeSet<Vec<&usize>>=BTreeSet::new();
    println!(" {}  {}  {:?}", &integers,&subarrays,&collection);
    let mut counter =0;
    for mut elt in (0..2usize.pow(integers as u32)).map(|i| {
         collection.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
             .map(|(_, element)| element).collect::<Vec<&usize>>()}) {    
        let elt_original = elt.to_owned();
        elt.sort();
        elt.dedup();
        if elt.len() <= subarrays && check.insert(elt_original.to_owned()) {
            println!("{:?}",&elt_original);
            counter+=1;
        }
    }
    println!("counter {}",counter);
}

