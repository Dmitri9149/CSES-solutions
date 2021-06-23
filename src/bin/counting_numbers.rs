//https://cses.fi/alon/task/1744/
use std::io::{BufRead};
use std::io;
use std::cmp;

pub fn read_lines() -> (usize,usize) {
    let stdin = io::stdin();
    let mut iter_line = stdin.lock().lines();
    let first_line = iter_line
        .next()
        .unwrap()
        .expect("failed to read first line");
    let mut fst_line_iter = first_line.split_whitespace();
    let aa = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    let bb = fst_line_iter
        .next()
        .unwrap()
        .parse::<usize>().unwrap();
    (aa, bb)
}

pub struct Dp {
   pub  dp:[[[[Option<usize>;19];10];2];2],
   pub num:Vec<usize>,
}
impl Dp {
    pub fn new() -> Dp {
        Dp {
            dp:[[[[None;19];10];2];2],
            num:Vec::with_capacity(19),
        }
    }
    pub fn num(&mut self, mut n:usize) {
        while n > 0 {
            self.num.push(n % 10);
            n /= 10;
        }
        self.num.reverse();
    }
    pub fn calc(&mut self,pos:usize,dig:usize,zeros:usize,range:usize) -> Option<usize> {
        if pos == self.num.len() {return Some(1)};
        if self.dp[range][zeros][dig][pos] != None {return self.dp[range][zeros][dig][pos]};
        let mut res = 0;
        let lmt = if zeros == 1 {9} else {self.num[pos]};
        let mut handle_zeros;
        let mut handle_range;
        for i in 0..=lmt {
            handle_zeros = if zeros == 1 || (i < lmt) {1} else {0};
            handle_range = if range == 1 || (i > 0) {1} else {0};
            if (i != dig || (range == 0  && i == 0)) {
                res += self.calc(pos +1, i,handle_zeros,handle_range).unwrap();
            }
        }
        self.dp[range][zeros][dig][pos] = Some(res);
        return Some(res);
    }
}

fn main() {
    let (mut aa, bb) = read_lines();
    let mut closure = |n| -> usize {
        let mut dp_xx = Dp::new();
        if n == 0 {return 1};
        dp_xx.num(n);
        dp_xx.calc(0,0,0,0).unwrap()
    };
    if aa == 0 {
        print!("{}", closure(bb));
    } else {
        print!("{}", closure(bb) - closure(aa-1));
    }
}
