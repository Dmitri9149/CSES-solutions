/*
There is a list of n numbers and two players who move alternately. On each move, a player removes either the first or last number from the list, and their score increases by that number. Both players try to maximize their scores.

What is the maximum possible score for the first player when both players play optimally?

Input

The first input line contains an integer n: the size of the list.

The next line has n integers x1,x2,…,xn: the contents of the list.

Output

Print the maximum possible score for the first player.

Constraints
1≤n≤5000
−109≤xi≤109
*/
// Some ideas are taken from this intersting 
// C++ solution : https://medium.com/intellectually-yours/cses-removal-game-d1a0c2753d8d
use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

pub struct Scores {
    scores:Vec<Vec<(i64,i64)>>,
}
impl Scores {
    pub fn new(coins:&Vec<i64>) -> Scores {
// size equal to the n in the task 
        let size = coins.len() -1;
// we will work with indices from 1... to size + 1
        let mut scores:Vec<Vec<(i64,i64)>> = vec![vec![(0,0);size+1];size+1];
// base scores , first player correspond to first position in (a,b)
        for i in 1..=size {
            scores[i][i] = (coins[i],0);
        }
        Scores {
            scores:scores,
        }
    }
}
pub fn read_lines() -> Vec<i64> {
    let stdin = io::stdin();
    let iter:SplitWhitespace;
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<i64>= Vec::with_capacity(number+1);
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read second line");
    iter = line.split_whitespace();
    vect.push(0);
    for elt in iter {
        vect.push(elt.parse::<i64>().unwrap());
    }
    vect
}

fn main() {
    let coins = read_lines();
    let size = coins.len() -1;
    let mut scores = Scores::new(&coins);
    let mut l = 1;// l is for the length of interval i..j
    while l < size {
        let mut i = 1;
        let mut j = 1+l;
        while j <= size {
            if coins[i] + scores.scores[i+1][j].1 >= coins[j] + scores.scores[i][j-1].1 {
                scores.scores[i][j] = (coins[i] + scores.scores[i+1][j].1, scores.scores[i+1][j].0);
            } else {
                scores.scores[i][j] = (coins[j] + scores.scores[i][j-1].1, scores.scores[i][j-1].0);

            }
            i+=1;
            j+=1;
        }
        l+=1;
    }
    print!("{}",scores.scores[1][size].0);
}
