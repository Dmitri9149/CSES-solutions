use std::io::{BufRead};
use std::io;
const MOD:usize = 1000000000 +7;
const SIZE:usize = 1000000;

fn main() {
    let mut dp_0 = [0;SIZE+1];
    let mut dp_1 = [0;SIZE+1];
    dp_0[1]=1;
    dp_1[1]=1;
    dp_0[2]=3;
    dp_1[2]=5;
    for i in 3..=SIZE {
        dp_0[i]=(dp_0[i-1]*2 + dp_1[i-1])%MOD;
        dp_1[i]=(dp_1[i-1]*4+dp_0[i-1])%MOD;
    }
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut seed:usize;
    let mut input;
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

