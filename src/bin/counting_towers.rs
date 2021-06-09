use std::io::{BufRead};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;
const MOD:usize = 1000000000 +7;

pub fn read_lines() -> (Vec<usize>,usize) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<usize> = Vec::with_capacity(number);
    let mut seed:usize=0;
    let mut input;
    for line in iter_line {
        input = line.expect("Failed to read line");
        seed = input.parse::<usize>().unwrap();
        vect.push(seed);
        number-=1;
        if number == 0 {
            break;
        }
    }
    (vect,number)
}

pub fn how_many(n:&usize) {
    let mut dp_0 = HashMap::with_capacity(*n);
    let mut dp_1 = HashMap::with_capacity(*n);
    if *n == 1 {
        println!("{}",&2);
        return ()
    } else if *n == 2 {
        println!("{}",&8);
        return ()
    } 
    dp_0.insert(2,3);
    dp_1.insert(2,5);
    let mut ins_0=3;
    let mut ins_1=5;
    for i in 3..=*n {
        ins_0 = (dp_0.get(&(i-1)).unwrap()*2 + dp_1.get(&(i-1)).unwrap())%MOD;
        ins_1 = (dp_1.get(&(i-1)).unwrap()*4+dp_0.get(&(i-1)).unwrap())%MOD;
        dp_0.insert(i,ins_0);
        dp_1.insert(i,ins_1);
    }
    println!("{}", (ins_0 + ins_1) % MOD);
    ()
}
fn main() {
    let (collection,tests) = read_lines();
    let mut count=0;
    let mut end = 0;
    for elt in collection.iter() {
        how_many(elt);
    }
//    println!("{:?}   {}",&collection,&tests);
//    println!(" if 6 {}", how_many(&6));
}

