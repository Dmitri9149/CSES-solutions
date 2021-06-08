//https://cses.fi/alon/task/1074
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> (usize,[usize;200000]) {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect=[0;200000];
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    let mut count = 0;
    for elt in iter {
        vect[count] = elt.parse::<usize>().unwrap();
        count+=1;
    }
/*
    if number != vect.len() {
        panic!("declared lenght and real lengths are different, panic!");
    }
*/
    (number,vect)
}

fn main() {
    let (size,mut data) = read_lines();
//    println!("unsorted data {:?}",data);
    data.sort();
//    println!("sorted data {:?}",data);
    let med = data[size / 2];
    let mut b:i64 = 0;
    for i in 200000-size.. 200000 {
        b+=(data[i] as i64-med as i64);
    }
/*
    for elt in data {
        b+= (elt as i64 - med as i64).abs()
    }
*/
    print!("{}",b);
}
