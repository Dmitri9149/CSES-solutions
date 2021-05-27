use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::convert::TryInto;

fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let line = iter_line
        .next()
        .unwrap()
        .expect("failed to read line");
    let mut iter:SplitWhitespace;
    iter = line.split_whitespace();
    let total = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let step = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut vect:Vec<u32>=(1..=total).collect();
    let mut iter1;
    let mut elt:u32;
    let mut vect1;;
    let mut quant:u32;
    let mut size:u32;
    let mut delta;
    loop {
        size = vect.len() as u32;
        if size >= (step +1) as u32 {

            delta = (size / (step+1))*(step+1);
                vect1 = vect[delta as usize..size as usize].to_owned();
            iter1 = vect.iter();
            for i in 0..delta {
                elt = *iter1.next().unwrap();
                if (i+1)%(step+1)==0 {
                    print!("{:?} ",elt);
                } else {
                    vect1.push(elt);
                }
            }
        } else if size == (step +1) as u32 {
            print!("{} ",&vect.pop().unwrap());
            vect1=vect;
        } else {
            quant = step -(step/size)*size;
            if quant == 0 {
                print!("{} ",&vect[0]);
                vect1=vect[1..].to_vec();
            } else {

                iter1 = vect.iter();
                    vect1 = vect[(quant+1) as usize ..size as usize].to_owned(); 
                for i in 0..quant {
                    elt = *iter1.next().unwrap();
                    vect1.push(elt);
                }
                elt = *iter1.next().unwrap();
                print!("{} ",elt);            

            }
        }
        if vect1.len() == 0 {
            break;
        }
        vect=vect1;
    }
}
