use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a: i64 = split.next().unwrap().parse().unwrap();
    print!("{} ",a);

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
}
