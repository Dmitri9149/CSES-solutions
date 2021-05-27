use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::convert::TryInto;

pub fn rec(size:u32,step:u32) -> u32 {
    let mut new:u32;

    if size == 1 {
    print!("{} ",1);
        return 1;
    }

    let new = (rec(size-1, step) + step-1) % size +1;
    print!("{} ",new);
    return(new);
}

fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read line");
    let mut iter:SplitWhitespace;
    iter = line.split_whitespace();
    let total = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let step = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    rec(total,step);
}
