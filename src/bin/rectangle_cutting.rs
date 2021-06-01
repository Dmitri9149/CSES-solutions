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
/*
    let mut count:usize=0;
    let mut left:usize=0;
    let mut right:usize=0;
*/
    let mut res:Vec<Vec<usize>>= Vec::with_capacity(aa+1);
    let mut internal:Vec<usize>=Vec::with_capacity(bb+1);
    for i in 0..=bb {
        internal.push(std::u32::MAX as usize);
    }
    for i in 0..=aa {
        res.push(internal.to_owned());
    }
    for  i in 0..=aa {
        for  j in 0..=bb {
            if i == j {
                res[i][j] = 0;
            } else {
//                res[i][j] = usize::MAX;
                for  k in 1..i {
                    res[i][j] = cmp::min(res[i][j], res[k][j]+res[i-k][j]+1);
                }
                for  k in 1..j {
                    res[i][j] = cmp::min(res[i][j], res[i][k]+res[i][j-k]+1);
                }
            }
        }
    }
    println!("{}",res[aa][bb]);
}

