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
se std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;


pub fn read_lines() {
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
                return ();
            }   
        }
    }
}


pub struct Perm {
    pub n:usize,
    pub line:String,
    pub permutation:String,
    pub chosen:[bool;8],
    pub se:BTreeMap<String,bool>
}

impl Perm {
    fn new(n:&usize
           ,line:&String
           ,perm:&String
           ,chosen:&[bool;8]
           ,se:&BTreeMap<String,bool>) -> Perm {
        Perm {
            n:n.to_owned(),
            line:line.to_owned(),
            permutation:perm.to_owned(),
            chosen:chosen.to_owned(),
            se:se.to_owned(),

        }
    }
    fn search(&mut self) {
        if self.permutation.len() == self.n {
        self.se
            .entry(self.permutation.to_string())
            .or_insert(true);
        } else {
            for i in 0..self.n {
                if self.chosen[i] {continue};
                self.chosen[i]=true;
                self.permutation.push(self.line.chars().nth(i).unwrap());
                self.search();
                self.chosen[i]=false;
                self.permutation = self.permutation[0..self.permutation.len()-1].to_string();
            }
        }
    }
}

fn main() {
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
    let mut ob:Perm = Perm::new(&n,&line,&permutation,&chosen,&se);
    ob.search();
    print!("{}\n",&ob.se.len());

    for (el,statement) in ob.se.iter() {
        print!("{}\n",&el);
    }
}
