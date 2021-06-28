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
//    let mut collection:[usize;200000] = [0;200000];
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
    for i in 0..values {
        seed = iter.next().unwrap().parse::<usize>().unwrap();
        collection.push(seed);
//        collection[i]=seed;
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
pub fn construct_mid_node(s:usize,e:usize) -> usize {
    s + (e-s) / 2 
}

pub struct SegmentTree {
    pub tree:Vec<usize>,
}

impl SegmentTree {
    pub fn new() -> SegmentTree {
        SegmentTree {
            tree:Vec::new(),
        }
    }
    pub fn from_array(&mut self, array:&Vec<usize>,start:usize,end:usize,current_node:usize) -> usize {
        let mut res;
        if start == end {
            res = array[start];
            self.tree[current_node]=res;
            return res
        }
        let mid = construct_mid_node(start,end);
        res = self.from_array(array,start,mid,current_node*2+1) + 
            self.from_array(array,mid+1,end,current_node*2 + 1);
            self.tree[current_node]=res;
            return res
    }
    pub fn construct_tree(&mut self, array:&Vec<usize>,end:usize) {
        self.from_array(array,0,end, 0);
    }
    pub fn get_sum_helper(&mut self,start:usize, end:usize
                        ,qstart:usize,qend:usize,current_node:usize) 
        -> usize {
            if qstart <= start && qend >= end {
                return self.tree[current_node];
            } else if end < qstart || start > qend {
                return 0;
            }
            let mid = construct_mid_node(start,end);
            return self.get_sum_helper(start, mid, qstart,qend,2*current_node +1) + 
                self.get_sum_helper(mid+1,end,qstart,qend,2*current_node +2);
    }
    pub fn update_value_helper(&mut self, start:usize
                             ,end:usize, index:usize, diff:usize, current_node:usize) {
        if index < start || index > end {
            return ();
        }
        self.tree[current_node] += diff;
        if end != start {
            let mid = construct_mid_node(start,end);
            self.update_value_helper(start,mid,index,diff,2*current_node +1);
            self.update_value_helper(mid+1,end,index,diff,2*current_node +2);
        }
    }
    pub fn update_value(&mut self, array:&mut Vec<usize>, values:usize, index:usize, new_value:usize) {
        if index < 0 || index > values -1 {
            panic!("Wrong index");
        }
        let diff = new_value - array[index];
        array[index]= new_value;
        self.update_value_helper(0,values-1,index,diff,0);
    }
    pub fn get_sum(&mut self,values:usize,qstart:usize,qend:usize) -> usize {
        if qstart < 0 || qend > values -1 || qstart > qend {
            panic!("Wrong start or end index");
        }

        return self.get_sum_helper(0,values-1,qstart,qend,0);
    }
}
fn main() {
    let (values,queries,mut collection,qrs) = read_lines();
    let mut segment_tree = SegmentTree::new();
    segment_tree.construct_tree(&collection,values);
    println!("{} {} {:?} {:?}",values,queries,collection,qrs);
}

