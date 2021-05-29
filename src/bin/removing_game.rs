use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub struct Scores {
    scores:Vec<Vec<(u32,u32)>>,
}
impl Scores {
    pub fn new(coins:&Vec<u32>) -> Scores {
        let size = coins.len();
        let mut scores:Vec<Vec<(u32,u32)>> = vec![vec![(0,0);size];size];
// base scores , first player correspond to first position in (a,b)
        for i in 1..=size {
            scores[i][i] = (coins[i],0);
        }
        Scores {
            scores:scores,
        }
    }
}
pub fn read_lines() -> Vec<u32> {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<u32>().unwrap();
    let mut vect:Vec<u32>= Vec::with_capacity(number as usize);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    for elt in iter {
        vect.push(elt.parse::<u32>().unwrap());
    }
    vect
}

fn main() {
    let mut coins = read_lines();
    let size = coins.len();
    let mut scores = Scores::new(&coins);
    let mut l = 1;// the length of interval i..=j
    let mut i =1;
    let mut j =i+l;
    while l < size {
        i = 1;
        j = 1+l;

        while j <= size {
            if (coins[i] + scores.scores[i+1][j].1 >= coins[j] + scores.scores[i][j-1].1) {
                scores.scores[i][j] = (coins[i] + scores.scores[i+1][j].1, scores.scores[i+1][j].0);
            } else {
                scores.scores[i][i] = (coins[i],0);

            }
            i+=1;
            j+=1;
        }
        l+=1;
    }
    print!("{}",scores.scores[1][size].0);
}
