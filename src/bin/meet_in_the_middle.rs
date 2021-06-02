//https://cses.fi/alon/task/2428
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;

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

fn summ(s: &[u32]) -> Vec<u32> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| *element).sum()
//                             .collect::<Vec<usize>>().sum()

     }).collect()
}
fn main() {
    let (total,sub_sum,set1,set2) = read_lines();
    let mut count:usize=0;
//    let mut left;
//    let mut right;
    if total == 1 && sub_sum == 1{
        println!("{}",sub_sum);
        return ()
    }
    let mut sums1:Vec<u32>= Vec::with_capacity(total/2);
    let mut sums2:Vec<u32>= Vec::with_capacity(total-total/2);

    sums1 = summ(&set1);
    sums1.sort();
    sums2 = summ(&set2);
    sums2.sort();

    let mut state; 
    let mut left_iter;
    if true {    //sums1.len() < sums2.len() {
        left_iter = sums1.iter();
        loop {
            match left_iter.next() {
                None => break,
                Some(x) => state = x,

            }
            if state > &sub_sum {
                break;
            }
            for elt in sums2.iter() {
                if state + elt == sub_sum {
                    count+=1;
                } else if state + elt > sub_sum {
                    break;
                }
            }            
        }
    }
//    println!("{:?}    {:?}",&sums1,&sums2);
    println!("{}",count);
}

