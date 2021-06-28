use std::io::{BufRead};
use std::io;
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::collections::BTreeMap;
//use segment_tree::SegmentPoint;
//use segment_tree::ops::Add;

pub fn read_lines() -> (usize,usize,HashMap<usize,usize>,HashMap<usize,[usize;3]>) {
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

    let mut collection:HashMap<usize,usize> = HashMap::with_capacity(values);
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
fn main() {
    let (values,queries,mut collection,qrs) = read_lines();
//    let mut tree = BTreeMap::new();
//    for (i,elt) in collection.iter() {
//        *tree
//            .entry(elt.to_owned())
//            .or_insert(0) +=1;
//    }
//    println!("Tree {:?}",tree);
    let mut query; 
    let mut state;
    for i in 0..queries {
        query = qrs.get(&i).unwrap(); 
        if query[0] == 1 {
            state = collection
                .get(&(query[1]-1))
                .unwrap()
                .to_owned();
            *collection
                .get_mut(&(query[1]-1))
                .unwrap() = query[2];
//            *tree
//                .entry(query[2])
//                .or_insert(0)+=1;
//            *tree
//                .entry(state)
//                .or_insert(0)-=1;

        } else {
            let mut sum =0;
            for elt in query[1]-1..=query[2]-1 {
                sum +=collection.get(&elt).unwrap();
            }
            println!("{}",sum);

        }
    }

/*
    let mut tree = SegmentPoint::build(collection, Add);
    let mut query; 
    for i in 0..queries {
        query = qrs.get(&i).unwrap(); 
        if query[0] == 1 {
            tree.modify(query[1],query[2]);
        } else {
            println!("{}",tree.query(query[1], query[2]));

        }
    }
*/
//    println!("{} {} {:?} {:?}",values,queries,collection,qrs);
}

