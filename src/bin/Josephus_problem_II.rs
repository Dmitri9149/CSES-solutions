use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;

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
        .parse::<usize>()
        .unwrap();
    let step = iter
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut vect:Vec<usize>=Vec::new();
    for i in 1..= total {
        vect
            .push(i);
    }
    let mut elt:usize;
    let mut vect1:Vec<usize>=Vec::new();
    let mut counter = 0;
    loop {
        let size = vect.len();
        if size >= step +1 {

            let rest = size - (size / (step+1))*(step+1);
            if rest != 0  {
                for i in 0..step {
                vect1 = vect[(size-rest)..size].to_owned();
                }
            }
//            println!("rest {} vect1 {:?}",&rest, &vect1);
            let mut iter1 = vect.iter();
            for i in 0..(size - rest) {
                elt = *iter1.next().unwrap();
                if (i+1)%(step+1)==0 {
                    print!("{:?} ",elt);
                } else {
//                    println!("push elt {}",&elt);
                    vect1.push(elt);
                }
            }
//        } else if size == step {
//            for elt in vect.iter() {
//                print!("{} ",elt);
//            }
        } else {
            let mut quant = step%size;
            let mut iter1 = vect.iter();

            let rest = size - (quant+1); 
            if rest != 0  {
                for i in 0..step {
                vect1 = vect[(size-rest)..size].to_owned();
                }
            }
            for i in 0..(size-rest) {
                elt = *iter1.next().unwrap();
                if (i+1)%(quant+1)==0  {
                    print!("{:?} ",elt);
                } else {
                    vect1.push(elt);
                }
            }
        }
        if vect1.len() == 0 {
            break;
        }
        vect=vect1;
        vect1=vec![];
/*
        println!("vect: {:?}", &vect);
        counter+=1;
        if counter == 8 {break;}
*/
    }
}
