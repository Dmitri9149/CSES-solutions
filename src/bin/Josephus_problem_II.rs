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
    let mut vect:Vec<usize>=Vec::with_capacity(100000);
    for i in 1..= total {
        vect
            .push(i);
    }
    let mut elt:usize;
    let mut vect1;;
    let mut quant;
    let mut size;
    loop {
        size = vect.len();
        if size >= step +1 {

/*
            let rest = size - (size / (step+1))*(step+1);
//            if rest != 0  {
                vect1 = vect[(size-rest)..size].to_owned();
//            }
//            println!("rest {} vect1 {:?}",&rest, &vect1);
*/

            let new_point = (size/(step+1))*(step+1);
            vect1 = vect[new_point..size].to_owned();

            let mut iter1 = vect.iter();
            for i in 0..new_point {
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
//            quant = step%size;
            quant = step -(step/size)*size;
//            println!("quant {}",&quant);
            let mut iter1 = vect.iter();

            let rest = size - (quant+1); 
//            if rest != 0  {
                vect1 = vect[(size-rest)..size].to_owned();
//            }
 
            for i in 0..quant {
                elt = *iter1.next().unwrap();
                vect1.push(elt);
            }
            elt = *iter1.next().unwrap();
            print!("{} ",elt);            
/*
            for i in 0..(size - rest) {
                elt = *iter1.next().unwrap();

                if (i+1)%(quant+1)==0  {
                    print!("{:?} ",elt);
                } else {
                    vect1.push(elt);
                }
            }
*/
        }
        if vect1.len() == 0 {
            break;
        }
        vect=vect1;
//        vect1=Vec::with_capacity(100000);
    }
}
