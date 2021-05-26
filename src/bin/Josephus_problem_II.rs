use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

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
        .parse::<usize>()
        .unwrap();
    let step = iter
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut vect:Vec<usize>=Vec::new();
    for i in 1..= total {
        vect
            .push(i);
    }
    let mut elt:usize;
    let mut vect1:Vec<usize>=Vec::new();
    loop {
        let size = vect.len();
        if size <= step{
            for elt in vect.iter() {
                print!("{:?} ",*elt);
            }
            break;
        }
        let rest = size % (step+1);
        if rest != 0  {
            for i in 0..step {
            vect1 = vect[(size-rest)..size].to_owned();
            }
        }
        let mut iter = vect.iter();
        for i in 0..(size - rest) {
            elt = *iter.next().unwrap();
            if (i+1)%(step+1)==0 {
                print!("{:?} ",elt);
            } else {
                vect1.push(elt);
            }
        }
        if vect1.len() == 0 {
            break;
        }
        vect=vect1;
        vect1=vec![];
    }
}
