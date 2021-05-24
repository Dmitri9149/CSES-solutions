/*
In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?

Input

The first input line has an integer n: the number of movies.

After this, there are n lines that describe the movies. Each line has two integers a and b: the starting and ending times of a movie.

Output

Print one integer: the maximum number of movies.
*/
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> BTreeMap<u64,u64> {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<u64>().unwrap();
    let mut tree:BTreeMap<u64,u64> = BTreeMap::new();
    let mut seed:(u64,u64)=(0,0);
    for line in iter_line {
        let input = line.expect("Failed to read line");
        iter = input.split_whitespace();
        seed.0 = iter.next().unwrap().parse::<u64>().unwrap();
        seed.1 = iter.next().unwrap().parse::<u64>().unwrap();
        match tree.get_mut(&seed.1) {
            Some(x) => if seed.0 > *x {*x=seed.0},
            None => {
                tree.insert(seed.1,seed.0);
            }
        }
        number-=1;
        if number == 0 {
            break;
        }
    }
    tree
}

fn main() {
    let tree = read_lines();
    let mut count=0;
    let mut end = 0;
    for (fst, snd) in tree.iter() {
        if snd >= &end {
            count+=1;
            end = *fst;
        }
    }
    println!("{}",count);
}

