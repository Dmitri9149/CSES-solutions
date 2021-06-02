https://cses.fi/alon/task/1744/
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::cmp;

pub fn read_lines() -> (u32,u32) {
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
        .parse::<u32>().unwrap();
    let bb = fst_line_iter
        .next()
        .unwrap()
        .parse::<u32>().unwrap();

    (aa, bb)
}
fn main() {
    let (aa, bb) = read_lines();
    let mut res = [[501; 501];501];
    if aa==bb {
        println!("{}",0);
        return ();
    }
    for  i in 0..=aa {
        for  j in 0..=bb {
            if i == j {
                res[i as usize ][j as usize ] = 0;
            } else {
                for  k in 1..i {
                    res[i as usize][j as usize] = cmp
                        ::min(res[i as usize][j as usize]
                              , res[k as usize][j as usize]+res[(i-k) as usize][j as usize]+1);
                }
                for  k in 1..j {
                    res[i as usize ][j as usize] = cmp
                        ::min(res[i as usize][j as usize]
                              , res[i as usize][k as usize]+res[i as usize][(j-k) as usize]+1);
                }
            }
        }
    }
    println!("{}",res[aa as usize ][bb as usize]);
}
