//https://cses.fi/alon/task/2428
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::cmp;

pub fn read_lines() -> (usize,usize) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let aa = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let bb = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    (aa, bb)
}

fn main() {
    let (aa, bb) = read_lines();
    if aa == 1 {
        print!("{}",bb-1);
    } else if bb ==1 {
        print!("{}",aa-1);
    } else if aa == bb {
        print!("{}",aa -1);
    }  


    let mut count:usize=0;
    let mut n = 1;
    let mut sizes:(usize, usize);
    sizes = (cmp::max(aa,bb),cmp::min(aa,bb));
    let mut sizes_new:(usize,usize);
// aa to be less than bb
    loop  {
        println!("sizes  {:?}",sizes);
        if sizes.0 == sizes.1 {
            print!("{}",count);
            break;
        }
        sizes_new=(sizes.0-sizes.1,sizes.1);
        sizes = (cmp::max(sizes_new.0,sizes_new.1),cmp::min(sizes_new.0,sizes_new.1));
        println!("after sizes new sizes are {:?}",sizes);
        n+=1;
        count+=1;
    }
}
