use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
pub fn read_lines() -> (usize,usize,Vec<usize>,HashMap<usize,[usize;3]>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let values = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let queries = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    let mut collection:Vec<usize> = Vec::with_capacity(values);
    let mut seed;
/*
    for line in iter_line {
        let input = line.expect("Failed to read last line");
        iter = input.split_whitespace();
        for _i in 0..machines {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection.push(seed);
        }
        break;
    }
*/
    let second_input = iter_line.next().unwrap().expect("Failed to read seconf line");
    iter = second_input.split_whitespace();
    for _i in 0..values {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection.push(seed);
    }
    let mut lines_number = 0;
    let mut qrs:HashMap<usize,[usize;3]> = HashMap::with_capacity(queries);
    for line in iter_line {
        let input = line.expect("Failed to read last line");
        iter = input.split_whitespace();
        let mut seeds:[usize;3]=[0;3];
        for i in 0..3 {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            seeds[i]=seed;
        }
        qrs.insert(lines_number,seeds);
        lines_number +=1;
        if lines_number == queries {break};
    }


    (values,queries,collection,qrs)
}

fn main() {
    let (values,queries,collection,qrs) = read_lines();
    println!("{} {} {:?} {:?}",values,queries,collection,qrs);
}

