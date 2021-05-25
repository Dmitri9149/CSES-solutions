use std::io::{BufRead};
use std::io;
use std::collections::BTreeMap;

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

    let mut flag:bool=false;
    let mut tree1:BTreeMap<usize,bool>= BTreeMap::new();
    loop {
        flag = false;
//        println!("tree in loop{:?}",&tree);
//        break;
        for (elt,cond) in tree.iter() {
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
                tree1.insert(*elt,*cond);
                flag=true;

//                print!("{:?} ",elt);
//                flag = false;  
            }

        }
/*        break;
        if tree1.len() == 0 {
            break;
        }
*/
        match &tree1.len() {
            0 => {break;},
            1=> {print!("{}",&tree1.iter().next().unwrap().0); break;},
            _=> (),
        }
        tree=tree1.to_owned();
//        println!("tree after assigning {:?}", &tree);
        tree1=BTreeMap::new();
/*
        println!("new tree {:?}", tree);
        break;
*/
    }


//    println!("number: {}",number);
//    println!("tree: {:?}",tree);
}
