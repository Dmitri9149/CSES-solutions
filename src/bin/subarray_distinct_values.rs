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
//    println!("integers {}", &integers);
    let subarrays = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
//    println!("subarrays {}", &subarrays);

    let mut collection:Vec<usize> = Vec::with_capacity(integers);
    let mut index = 0;
    let mut seed = 0;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for i in 0..integers {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
//            println!("index {} and seed {}", &index, &seed);
            collection.push(seed);
        }
        break;
    }
    (integers, subarrays,collection)
}

fn main() {
    let (integers, subarrays,collection) = read_lines();
    let mut check:BTreeSet<Vec<&usize>>=BTreeSet::new();
    let mut candidate;
    let mut counter:usize=0;
    for i in 0..integers-1 {
        for j  in  i+1..=integers {
            candidate = collection[i..j].to_vec();
            candidate.sort();
            candidate.dedup();
            if candidate.len() <= subarrays {
                counter+=1;
//                println!("i {} and j {}", i, j);
//                println!("candidate: {:?}", candidate);
            } else {
                break;
            }          
        }   
    }
    println!("{}",counter+1);
}

