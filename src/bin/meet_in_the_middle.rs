//https://cses.fi/alon/task/1628/
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::BTreeMap;

pub fn read_lines() -> (usize,usize,Vec<usize>,Vec<usize>) {
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
        .parse::<usize >().unwrap();

    let mut collection1:Vec<usize> = Vec::with_capacity(integers);
    let mut collection2:Vec<usize > = Vec::with_capacity(integers);
    let mut seed;
    for line in iter_line {
        let input = line.expect("Failed to last line");
        iter = input.split_whitespace();
        for _i in 0..(integers/2) {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection1.push(seed);
        }
        for _i in (integers/2)..integers {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection2.push(seed);
        }

        break;
    }
    (integers, subarrays,collection1,collection2)
}

fn summ(s: &[usize]) -> BTreeMap<usize,usize> {
    let mut tree:BTreeMap<usize,usize> = BTreeMap::new();
    for elt in (0..2usize.pow(s.len()as u32)).map(|i| {
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
    let mut count:usize=0;
    if total == 1 && sub_sum == 1{
        println!("{}",sub_sum);
        return ()
    }
    let sums1 = summ(&set1);
    let sums2 = summ(&set2);
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
    println!("{}",count);
}

