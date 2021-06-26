use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;

//const MAXSIZE:usize= 2*2_usize.pow(200000f64.log2().ceil() as u32);
const MAXSIZE:usize = 524288 -1; // calculated from the 200000 array size
pub fn read_lines() -> (usize,usize,Vec<usize>,HashMap<usize,[usize;3]>) {
    let stdin = io::stdin();
    let mut iter:SplitWhitespace; 
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let values = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let queries = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();

    let mut collection:Vec<usize> = Vec::with_capacity(values);
    let mut seed;
/*
    for line in iter_line {
        let input = line.expect("Failed to read last line");
        iter = input.split_whitespace();
        for _i in 0..machines {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            collection.push(seed);
        }
        break;
    }
*/
    let second_input = iter_line.next().unwrap().expect("Failed to read seconf line");
    iter = second_input.split_whitespace();
    for _i in 0..values {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection.push(seed);
    }
    let mut lines_number = 0;
    let mut qrs:HashMap<usize,[usize;3]> = HashMap::with_capacity(queries);
    for line in iter_line {
        let input = line.expect("Failed to read last line");
        iter = input.split_whitespace();
        let mut seeds:[usize;3]=[0;3];
        for i in 0..3 {
            seed = iter.next().unwrap().parse::<usize>().unwrap();
            seeds[i]=seed;
        }
        qrs.insert(lines_number,seeds);
        lines_number +=1;
        if lines_number == queries {break};
    }


    (values,queries,collection,qrs)
}
pub fn constructMidNode(s:usize,e:usize) -> usize {
    s + (e-s) / 2 
}

pub struct SegmentTree {
    pub tree:[usize;MAXSIZE],
}

impl SegmentTree {
    pub fn new() -> SegmentTree {
        SegmentTree {
            tree:[0;MAXSIZE],
        }
    }
    pub fn from_array(&mut self,arr:&[usize;200000],start:usize,end:usize,current_node:usize) -> usize {
        let mut res;
        if start == end {
            res = arr[start];
            self.tree[current_node]=res;
            return res
        }

        let mid = constructMidNode(start,end);
        res = self.from_array(arr,start,mid,current_node*2+1) + 
            self.from_array(arr, mid+1,end,current_node*2 + 1);
            self.tree[current_node]=res;
            return res
    }
    pub fn construct_tree(&mut self,arr:&[usize;200000]) {
        self.from_array(arr,0,200000 - 1, 0);
    }
}


fn main() {
    let (values,queries,collection,qrs) = read_lines();
    println!("{} {} {:?} {:?}",values,queries,collection,qrs);
}

