use std::io::{BufRead};
use std::collections::HashMap;
use std::io;
const MOD:usize = 1000000000 +7;

pub fn read_lines() -> (Vec<usize>,usize) {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut vect:Vec<usize> = Vec::with_capacity(number);
    let mut seed:usize;
    let mut input;
    let mut biggest=1; 
    for line in iter_line {
        input = line.expect("Failed to read line");
        seed = input.parse::<usize>().unwrap();
        vect.push(seed);
        if seed > biggest {
            biggest = seed;
        }
        number-=1;
        if number == 0 {
            break;
        }
    }
    (vect,biggest)
}

pub fn how_many(n:&usize) -> (HashMap<usize,usize>,HashMap<usize,usize>){
    let mut dp_0 = HashMap::with_capacity(n+1);
    let mut dp_1 = HashMap::with_capacity(n+1);
    dp_0.insert(1,1);
    dp_1.insert(1,1);
    dp_0.insert(2,3);
    dp_1.insert(2,5);
    let mut ins_0;
    let mut ins_1;
    for i in 3..=*n {
        ins_0 = (dp_0.get(&(i-1)).unwrap()*2 + dp_1.get(&(i-1)).unwrap())%MOD;
        ins_1 = (dp_1.get(&(i-1)).unwrap()*4+dp_0.get(&(i-1)).unwrap())%MOD;
        dp_0.insert(i,ins_0);
        dp_1.insert(i,ins_1);
    }
    (dp_0,dp_1)
}
fn main() {
    let (collection,biggest) = read_lines();
    let mut res;
    let (dp_0, dp_1) = how_many(&biggest);
    for elt in collection.iter() {
        res = (dp_0.get(elt).unwrap() + dp_1.get(elt).unwrap())%MOD;
        println!("{}",res);
    }
}

