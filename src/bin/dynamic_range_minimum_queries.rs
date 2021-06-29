use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::cmp;
//use segment_tree::SegmentPoint;
//use segment_tree::ops::Add;

const MAXSIZE:usize = 524288 -1; // tree max size calculated from the 200000 max input size
const MAXVAL:usize = usize::MAX;
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
    let second_input = iter_line.next().unwrap().expect("Failed to read seconf line");
    iter = second_input.split_whitespace();
    for i in 0..values {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection.insert(i,seed);
//        collection.push(seed);
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
pub fn construct_mid_node(start:usize,end:usize) -> usize {
    start + (end-start) / 2 
}
#[derive(Debug)]
pub struct SegmentTree {
    pub tree:Vec<usize>,
}
impl SegmentTree {
    pub fn new() -> SegmentTree {
        SegmentTree {
            tree:vec![0;MAXSIZE],
        }
    }
    pub fn from_array(&mut self, array:&Vec<usize>,start:usize,end:usize,current_node:usize) -> usize {
        let res;
        if start == end {
            res = array[start];
            self.tree[current_node]=res;
            return res
        }
        let mid = construct_mid_node(start,end);
        res = cmp::min(self.from_array(array,start,mid,current_node*2 + 1), 
            self.from_array(array,mid+1,end,current_node*2 + 2));
            self.tree[current_node]=res;
            return res
    }
    pub fn construct_tree(&mut self, array:&Vec<usize>, n:usize) {
        self.from_array(array,0,n-1,0);
    }
    pub fn get_min_helper(&mut self,start:usize, end:usize
                        ,qstart:usize,qend:usize,current_node:usize) 
        -> usize {
            if qstart <= start && qend >= end {
                return self.tree[current_node];
            }
            if end < qstart || start > qend {
                return MAXVAL;
            }
            let mid = construct_mid_node(start,end);
            return cmp::min(self.get_min_helper(start, mid, qstart,qend,2*current_node +1),
                self.get_min_helper(mid+1,end,qstart,qend,2*current_node +2));
    }
    pub fn update_value_helper(&mut self, start:usize
                             ,end:usize, index:usize, new_value:usize, current_node:usize) {
        if index < start || index > end {
            return ();
        }
        if start == end  {
            self.tree[current_node] = new_value;
            return ();
        }
        let mid = construct_mid_node(start,end);
        if index <= mid {
            self.update_value_helper(start,mid,index,new_value,2*current_node +1);
        } else {
            self.update_value_helper(mid+1,end,index,new_value,2*current_node +2);
        }

        self.tree[current_node]=cmp::min(self.tree[2*current_node+1]
                                         ,self.tree[2*current_node + 2]);
    }
    pub fn update_value(&mut self, array:&mut Vec<usize>, values:usize, index:usize, new_value:usize) {
        if index > values -1 {
            panic!("Wrong index");
        }
        array[index]= new_value;
        self.update_value_helper(0,values-1,index,new_value,0);
    }
    pub fn get_min(&mut self,values:usize,qstart:usize,qend:usize) -> usize {
        if qend > values -1 || qstart > qend {
            panic!("Wrong start or end index");
        }
        return self.get_min_helper(0,values-1,qstart,qend,0);
    }
}
fn main() {
    let (values,queries,mut collection,qrs) = read_lines();
    let mut segment_tree = SegmentTree::new();
    segment_tree.construct_tree(&mut collection,values);
    let mut query;
    for i in 0..queries {
        query = qrs.get(&i).unwrap();
        if query[0] == 1 {
            segment_tree.update_value(&mut collection, values, query[1]-1,query[2]);
//            println!("collection {:?}",collection);
//            println!("tree {:?}",segment_tree);
        } else {
            println!("{}",segment_tree.get_min(values,query[1]-1,query[2]-1));
        }
    }
}

