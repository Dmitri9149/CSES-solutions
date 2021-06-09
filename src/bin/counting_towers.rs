use std::io::{BufRead};
use std::collections::HashMap;
use std::io;
const MOD:usize = 1000000000 +7;


pub fn how_many(n:&usize) -> ([usize;1000002],[usize;1000002]){
    let mut dp_0 = [0;1000002];
    let mut dp_1 = [0;1000002];
    dp_0[1]=1;
    dp_1[1]=1;
    dp_0[2]=3;
    dp_1[2]=5;
    let mut ins_0;
    let mut ins_1;
    for i in 3..=*n {
        ins_0 = (dp_0[i-1]*2 + dp_1[i-1])%MOD;
        ins_1 = (dp_1[i-1]*4+dp_0[i-1])%MOD;
        dp_0[i]=ins_0;
        dp_1[i]=ins_1;
    }
    (dp_0,dp_1)
}
fn main() {
    let (dp_0, dp_1) = how_many(&1000000);
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut seed:usize;
    let mut input;
    let mut biggest=1; 
    let mut res;
    for line in iter_line {
        input = line.expect("Failed to read line");
        seed = input.parse::<usize>().unwrap();
        res = (dp_0[seed] + dp_1[seed])%MOD;
        println!("{}",res);
        number-=1;
        if number == 0 {
            break;
        }
    }
}

