use std::io::{BufRead};
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let number = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line")
        .parse::<usize>().unwrap();
    let mut tree:Vec<usize>=Vec::new();
    for i in 1..=number {
        tree
            .push(i);
    }
    let mut flag:bool = false;
    let mut elt:usize;
    let mut tree1:Vec<usize>=Vec::new();
    loop {
        if tree.len() == 1{
            print!("{:?} ",&tree[0]);
            break();

        }
        let size = tree.len();
        let rest = size % 2;
        if rest != 0  {
            tree1.push(tree.pop().unwrap())
        }
        let mut iter = tree.iter();
        for _i in 0..(size - rest) {
            elt = *iter.next().unwrap();
            if flag == true {
                print!("{:?} ",elt);
                flag = false;  
            } else {
                tree1.push(elt);
                flag=true;
            }
        }
        if tree1.len() == 0 {
            break;
        }
        tree=tree1;
        tree1=vec![];
    }
}
