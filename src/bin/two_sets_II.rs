//https://cses.fi/alon/result/2273368/
use std::io::{BufRead};
use std::{io,cmp};

//some ideas from here https://www.youtube.com/watch?v=TOsD3BkIKoQ  (in c++)
const MOD:usize = 1000000000 +7;
fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut res:Vec<Vec<usize>>=Vec::with_capacity(number+1);
    let size = cmp::max(2,number*number);
    let mut interm:Vec<usize>=Vec::with_capacity(size/2);
    for _i in 0..size/2 {
        interm.push(0);
    }
    for _i in 0..number+1 {
        res.push(interm.to_owned());
    }
    res[0][0]=1;
    for i in 1..=number {
        for x in 0..=(number*(number+1))/4 {
            if x < i {
                res[i][x]=res[i-1][x]%MOD;
            } else {
                res[i][x]=(res[i-1][x-i] + res[i-1][x])%MOD;
            }
        }   
    }
    if (number*(number+1)%4) != 0 {
        print!("{}",0);
    } else {
        print!("{}",(res[number][number*(number+1)/4]*500000004)%MOD);
    }
}
