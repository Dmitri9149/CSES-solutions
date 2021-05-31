// https://cses.fi/alon/task/1164
//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;

pub fn read_lines() -> (Vec<(usize,usize)>,Vec<(usize,usize)>,usize) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut start:Vec<(usize,usize)> = Vec::with_capacity(number);
    let mut end:Vec<(usize,usize)> = Vec::with_capacity(number);
    let mut seed:(usize,usize)=(0,0);
    let mut index = 0;
    for line in iter_line {
        let input = line.expect("Failed to read line");
        iter = input.split_whitespace();
        seed.0 = iter.next().unwrap().parse::<usize>().unwrap();
        seed.1 = iter.next().unwrap().parse::<usize>().unwrap();
        start.push((seed.0,index));
        end.push((seed.1,index));
        index+=1;
        if index  == number {
            break;
        }
    }
    (start,end,number)
}

fn main() {
    let (mut input, mut output, num) = read_lines();
    let mut res:Vec<usize> = Vec::with_capacity(num);
    for _i in 0..num {
        res.push(0);
    }

    let mut free:BTreeMap<usize,bool> = BTreeMap::new();
    for i in 0..num {
        free.insert(i,true);
    }
    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    output.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut k;
    let mut j =0;
    for i in 0..num {
        while j < num && output[j].0 < input[i].0 {
            free.insert(res[output[j].1],true);
            j+=1;
        }
        k = *free.iter().next().unwrap().0;
        res[input[i].1] = k;
        free.remove(&k);    
    }
    let max = res.iter().max().unwrap() +1;
    println!("{}",&max);
    for elt in res.iter() {
        print!("{} ",elt + 1 );
    }
}

