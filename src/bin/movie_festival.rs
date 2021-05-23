/*
In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?

Input

The first input line has an integer n: the number of movies.

After this, there are n lines that describe the movies. Each line has two integers a and b: the starting and ending times of a movie.

Output

Print one integer: the maximum number of movies.
*/
use std::io::{BufRead, BufReader};
//use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;
use std::cmp;
use std::convert::TryInto;


pub fn read_lines() -> Vec<(u64,u64)> {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<u64>().unwrap();
    let mut vect:Vec<(u64,u64)> = vec![];
    for line in iter_line {
        let input = line.expect("Failed to read line");
        iter = input.split_whitespace();
        let mut seed:(u64,u64)=(0,0);
        seed.0 = iter.next().unwrap().parse::<u64>().unwrap();
        seed.1 = iter.next().unwrap().parse::<u64>().unwrap();
        vect.push(seed);
        number-=1;
        if number == 0 {
            break;
        }
    }
    vect
}

pub struct Perm {
    pub intervals:Vec<(u64,u64)>,
    pub how_many:u64,
    pub perm:Vec<(u64,u64)>,
    pub chosen:[bool;200000],
}

impl Perm {
    fn new(intervals:&Vec<(u64,u64)>
           ,how_many:&u64
           ,perm:&Vec<(u64,u64)>
           ,chosen:&[bool;200000]
           )-> Perm {
        Perm {intervals:intervals.to_owned()
            ,how_many:how_many.to_owned()
            ,perm:perm.to_owned()
            ,chosen:chosen.to_owned()
        }
    }
    fn search(&mut self,k:&u64) {
        if *k  == self.intervals.len().try_into().unwrap(){
//            println!("Edge condition, k:{}, how_many: {}",&k,&self.how_many);
//            println!("Perm:{:?}",self.perm);
            let mut zeros = 0;
            for x in self.perm.iter() {
                if *x == (0,0) {
                    zeros+=1;
                }
            }
            let not_zeros = self.intervals.len()-zeros;
            if not_zeros > self.how_many.try_into().unwrap(){
                self.how_many = not_zeros.try_into().unwrap();
            }
//*****************************
//            println!("Edge condition, after if... how_many:{}",self.how_many);
        } else {
            for i in 0..self.intervals.len() {
                if self.chosen[i] {continue};
                self.chosen[i]=true;
//                println!("perm before loop: {:?}", self.perm);
                let mut flag = false;
                for x in self.perm.iter() {
                    if ((x.1-x.0)+(self.intervals[i].1 - self.intervals[i].0))
                        > (cmp::max(x.1,self.intervals[i].1)-cmp::min(x.0,self.intervals[i].0)) {
                        flag = true;
                        break;
                    }
                }
                if flag != true {
                    self.perm.push(self.intervals[i]);
                } else {
                    self.perm.push((0,0));
                }
//                println!("i: {}, k: {}", i, &k); 
//                println!("Perm:{:?}",self.perm);

                self.search(&(k+1));
                self.chosen[i]=false;
//                self.perm = self.perm[0..self.perm.len()-1];
                self.perm.pop();
//                println!("end of for loop with i = {} and k = {}\n",&i, &k);
            }
        }
//        println!("out of function with k = {}\n", &k);
    }
}

fn main() {
    let vect = read_lines();
    let chosen: [bool;200000] = [false;200000];
    let mut ob:Perm = Perm::new(&vect,&0,&vec![],&chosen);
    ob.search(&0);
    print!("{}\n",ob.how_many);
}
