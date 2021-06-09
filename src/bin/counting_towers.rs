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

pub fn read_lines() -> (Vec<usize>,usize) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<usize> = Vec::with_capacity(number);
    let mut seed:usize=0;
    let mut input;
    for line in iter_line {
        input = line.expect("Failed to read line");
        seed = input.parse::<usize>().unwrap();
        vect.push(seed);
        number-=1;
        if number == 0 {
            break;
        }
    }
    (vect,number)
}

fn main() {
    let (collection,tests) = read_lines();
    let mut count=0;
    let mut end = 0;
    println!("{:?}   {}",&collection,&tests);
}

