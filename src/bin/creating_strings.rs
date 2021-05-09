use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line)
        .unwrap();
    let size = line.len();
    if (size < 1 ) || (size > 8 ) {
        panic!("Not valid input string size! Panic!")
    }

//    let mut a: i64 = split.next().unwrap().parse().unwrap();
    print!("{} ",line);
/*
    while a != 1 && 1 < a {
        if a % 2 == 0 {
            a = a/2;
        } else {
            a = a*3 +1
        }

        if 1 <= a {
            print!("{} ",a);
        }
    }
*/
    let set:HashSet<char> = HashSet::with_capacity(size);
    for ch in line.chars() {
        
    }

    let collector:HashSet<char> = HashSet::new();
    for lenght in 

}
