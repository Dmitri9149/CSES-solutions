// https://cses.fi/alon/task/1164
//
use std::io::{BufRead};
use std::collections::BTreeMap;
use std::io;
use std::str::SplitWhitespace;
use std::convert::TryInto;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::cmp;

pub fn read_lines() -> (Vec<((u32,u32),u32)>,usize) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let mut number = iter_line
        .next()
        .unwrap()
        .expect("failed to read next line")
        .parse::<usize>().unwrap();
    let mut data:Vec<((u32,u32),u32)> = Vec::with_capacity(number.try_into().unwrap());
    let mut seed:(u32,u32)=(0,0);
    let mut index = 0;
    for line in iter_line {
        let input = line.expect("Failed to read line");
        iter = input.split_whitespace();
        seed.0 = iter.next().unwrap().parse::<u32>().unwrap();
        seed.1 = iter.next().unwrap().parse::<u32>().unwrap();
        data.push(((seed.0,seed.1),index));
        index+=1;
        if index  == number.try_into().unwrap() {
            break;
        }
    }
    (data,number)
}

fn main() {
    let (mut data, number) = read_lines();
    let mut res:Vec<u32> = Vec::with_capacity(number);
    data.sort_by(|a, b| a.0.0.partial_cmp(&b.0.0).unwrap());
    let mut quantity = 0;
    let mut last_room = 0;
    let mut pq = PriorityQueue::<u32,Reverse<u32>>::new();
    let mut min:(&u32,&Reverse<u32>)=(&0,&Reverse(0));
    for i in 0..number {
        if pq.is_empty() {
            last_room+=1;
            pq.push(data[i as usize].0.1 ,Reverse(last_room));
            res[data[i as usize].1 as usize ] = last_room;
        } else {
            min = pq[pq.len()];
            if min.1 < &Reverse(data[i as usize].0.0) {
                pq.pop();
                pq.push(data[i as usize].0.1, Reverse(min.0.to_owned()));
                res[data[i as usize].1 as usize] = *min.0;
            } else {
                last_room+=1;
                pq.push(data[i].0.1,Reverse(last_room));
                res[data[i as usize].1 as usize] = last_room;
            }
        }
        quantity = cmp::max(quantity, pq.len());
    }
    println!("{}",quantity);
}

