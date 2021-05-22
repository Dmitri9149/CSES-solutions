/*
In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?

Input

The first input line has an integer n: the number of movies.

After this, there are n lines that describe the movies. Each line has two integers a and b: the starting and ending times of a movie.

Output

Print one integer: the maximum number of movies.
*/
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;


pub fn read_lines() -> Vec<(u32,u32)> {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line)
        .unwrap();
// remove the trailing '\n'
    line.pop();
    let mut number = line.parse::<u32>().unwrap();
    let stdin = io::stdin();
    let mut vect:Vec<(u32,u32)> = vec![];
    let mut iter:SplitWhitespace; 
    loop {
        // Iterate over all lines that will be inputted
        for line in stdin.lock().lines() {
            let input = line.expect("Failed to read line");
            iter = input.split_whitespace();
            let mut seed:(u32,u32)=(0,0);
            seed.0 = iter.next().unwrap().parse::<u32>().unwrap();
            seed.1 = iter.next().unwrap().parse::<u32>().unwrap();
            vect.push(seed);
            number-=1;
            if number ==0 {
                return vect;
            }   
        }
    }
}

pub struct Perm {
    pub n:u32,
    pub k:u32,
    pub vect:Vec<(u32,u32)>,
    pub intervals:Vec<(u32,u32)>,
    pub how_many:u32,
    pub current:u32,
    pub chosen:[bool;200000],
    pub prev:(u32,u32)
}

impl Perm {
    fn new(n:&u32
           ,k:&u32
           ,vect:&Vec<(u32,u32)>
           ,intervals:&Vec<(u32,u32)>
           ,how_many:&u32
           ,current:&u32
           ,chosen:&[bool;200000]
           ,prev:&(u32,u32)
           ) -> Perm {
        Perm {
            n:n.to_owned()
            ,k:k.to_owned()
            ,vect:vect.to_owned()
            ,intervals:intervals.to_owned()
            ,how_many:how_many.to_owned()
            ,current:current.to_owned()
            ,chosen:chosen.to_owned()
            ,prev:prev.to_owned()

        }
    }
    fn search(&mut self) {
        if self.k == self.n-1 {
            if self.current > self.how_many {
                self.how_many = self.current;
            }
        } else {
            for i in 0..self.n {
                if self.chosen[i] {continue};
                self.chosen[i]=true;
                if self.prev.1 < self.vect[i].1 {
                    self.current+=1;
                }
                self.prev = self.vect[i];
                self.k+=1;
                self.search();
                self.chosen[i]=false;
                self.prev = (0,0);
                self.current=0;
                self.k = 0;
            }
        println!("{}",&self.how_many);
        }
    }
}

fn main() {
/*
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    let se:BTreeMap<String,bool> = BTreeMap::new();
    let permutation = "".to_string();
    let chosen: [bool;8] = [false;8];

    input.read_line(&mut line)
        .unwrap();
// remove the trailing '\n'
    line.pop();
// max size of input -> see task constrain
    let n_max = 8;
    let n = line.len();
    if (n < 1 ) || (n > n_max ) {
        panic!("Not valid input string size! Panic!")
    }
*/  
    let vect = read_lines();
    let chosen: [bool;200000] = [false;200000];
    let mut ob:Perm = Perm::new(
        &0
        ,&vect.len()
        ,&vect
        ,&0
        ,&0
        ,&0
        ,&chosen
        ,&(0,0));
    ob.search();
/*
    print!("{}\n",&ob.se.len());

    for (el,statement) in ob.se.iter() {
        print!("{}\n",&el);
    }
*/
}
