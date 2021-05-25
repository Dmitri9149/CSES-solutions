use std::io::{BufRead};
use std::io;
use std::collections::BTreeMap;
use std::iter::Rev;
use std::collections::VecDeque;



fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();

    let range = 0..=number;




    let mut tree:VecDeque<usize>=VecDeque::new();
    for i in 1..=number {
        tree
            .push(i);
    }
//    println!("number: {}",number);
//    println!("tree: {:?}",&tree);


    loop {
        let closure_a = |mut flag|-> VecDeque {
        let closure = |mut flag| {
            let mut tr = VecDeque::new();
            for elt in tree1.iter() {
    //            println!("flag before if {}",flag);
    //            println!("elt before if {}",elt);
                    
                if flag == true {
                    print!("{:?} ",&elt);
                    flag = false;  

    //                println!("flag in insert {}",flag);
    //                tree1.insert(*elt,*cond);
    //                flag=true;
    //                println!("flag in insert {}",flag);
                } else {
    //                println!(" elt in insert {}",&elt);
                    tr.push(*elt);
                    flag=true;

    //                print!("{:?} ",elt);
    //                flag = false;  
                }
            }
//            println!("closure_b: {:?}",&tr);
            tr
        };
//        let mut tree1:BTreeMap<usize,bool>=BTreeMap::new();
            tree1=closure_a(false);
//        print!("after closure call");
        let size = &tree1.keys().len();
//        println!("size :{:?}",size);
        match *size {
            0 => {break;},
            1=> {print!("{}",&tree1.iter().next().unwrap().0); break;},
            _=> {},
        }
//        print!("after match)");
//        break;
    }
}
