use std::io::{BufRead};
use std::io;
use std::collections::BTreeMap;
use std::iter::Rev;



fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut tree:BTreeMap<usize,bool>= BTreeMap::new();
    for i in 1..=number {
        tree
            .entry(i)
            .or_insert(true);
    }
//    println!("number: {}",number);
//    println!("tree: {:?}",&tree);

// 'true'  if iterate 'to the left' , in normal way
// 'false' if we will iterate 'to the right' , i.e. will use Rev<> iterator
    let mut direction:bool=true;
    let mut tree1=tree;
/*
    let tree_b:BTreeMap<usize,bool>= BTreeMap::new();
    let mut tree_iter_b=tree_b.into_iter().rev();
    */
//    let mut tree1:BTreeMap<usize,bool>= BTreeMap::new();
    loop {
        let closure_a = |mut flag|-> BTreeMap<usize,bool> {
            let mut tr = BTreeMap::new();
            for (elt,cond) in tree1.iter() {
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
                    tr.insert(*elt,*cond);
                    flag=true;

    //                print!("{:?} ",elt);
    //                flag = false;  
                }
            }
//            println!("closure_a: {:?}",&tr);
            tr

        };
        let closure_b = |mut flag| {
            let mut tr = BTreeMap::new();
            for (elt,cond) in tree1.iter() {
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
                    tr.insert(*elt,*cond);
                    flag=true;

    //                print!("{:?} ",elt);
    //                flag = false;  
                }
            }
//            println!("closure_b: {:?}",&tr);
            tr
        };
//        let mut tree1:BTreeMap<usize,bool>=BTreeMap::new();
        if direction == true {
            tree1=closure_a(false);
            direction = false;
//            println!("tree1: {:?}, direction: {:?}, size: {:?}", &tree1, &direction,&tree1.keys().len());
        } else {
            tree1=closure_a(false);
//            println!("tree1: {:?}, direction: {:?}, size: {:?}", &tree1, &direction,&tree1.keys().len());
            direction = true;
        }
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
