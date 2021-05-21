use std::io::{BufRead, BufReader};
//use std::collections::HashSet;
use std::collections::BTreeMap;




pub struct Perm {
    pub n:usize,
    pub line:String,
    pub permutation:String,
    pub chosen:[bool;8],
    pub se:BTreeMap<String,bool>
}

impl Perm {
    fn new(n:&usize
           ,line:&String
           ,perm:&String
           ,chosen:&[bool;8]
           ,se:&BTreeMap<String,bool>) -> Perm {
        Perm {
            n:n.to_owned(),
            line:line.to_owned(),
            permutation:perm.to_owned(),
            chosen:chosen.to_owned(),
            se:se.to_owned(),

        }
    }
    fn search(&mut self) {
        if self.permutation.len() == self.n {
//            println!("length {}", &self.permutation.len());
//            println!("will be inserted {}\n", &self.permutation.to_string());
//            self.se.insert(self.permutation.to_string());
        self.se
            .entry(self.permutation.to_string())
            .or_insert(true);
        } else {
            for i in 0..self.n {
                if self.chosen[i] {continue};
                self.chosen[i]=true;
                self.permutation.push(self.line.chars().nth(i).unwrap());
                self.search();
                self.chosen[i]=false;
                self.permutation = self.permutation[0..self.permutation.len()-1].to_string();
            }
        }
/*
        println!("{}",&self.se.len());
        for el in self.se.iter() {
            println!("{}",&el);
        }
*/
    }
}

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    let se:BTreeMap<String,bool> = BTreeMap::new();
    let permutation = "".to_string();
    let chosen: [bool;8] = [false;8];

    input.read_line(&mut line)
        .unwrap();
// remove the trailing '\n'
    line.pop();
// max size of input -> see task constrain
    let n_max = 8;
    let n = line.len();
    if (n < 1 ) || (n > n_max ) {
        panic!("Not valid input string size! Panic!")
    }
    let mut ob:Perm = Perm::new(&n,&line,&permutation,&chosen,&se);
    ob.search();
    print!("{}\n",&ob.se.len());

    for (el,statement) in ob.se.iter() {
        print!("{}\n",&el);
    }
}
