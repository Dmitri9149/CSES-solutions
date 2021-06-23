use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
//use std::convert::TryInto;
 
pub fn read_lines() -> (usize,Vec<u32>) {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<u32>= Vec::with_capacity(number);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<u32>().unwrap());
    }
    (number,vect)
}
 
fn main() {
    let (size, data) = read_lines();
    let mut stack:Vec<(u32,u32)> = Vec::with_capacity(size);
    for (i,elt) in data.iter().enumerate() {
        let mut length;
        let mut last;
        loop {
            length = stack.len();
            if length == 0 {
                print!("{} ",0);
                break;
            }
            last = stack[length-1];
            if last.1 >= *elt {
                stack.pop();
            } else {
                print!("{} ",last.0+1);
                break;
            }
        }
//        while stack.len() > 0 && stack[stack.len()-1].1 >= *elt {
//            stack.pop();
//        }
//        if stack.len() == 0 {
//            print!("{} ",0);
//        } else {
//            print!("{} ",stack[stack.len()-1].0+1);
//        }
//        stack.push((i.try_into().unwrap(),*elt));
        stack.push((i as u32,*elt));
    }
}
