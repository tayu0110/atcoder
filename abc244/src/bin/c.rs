#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(std::io::BufReader::new(stdin.lock()));
    input! {from &mut source, n: usize}

    let mut set = std::collections::HashSet::new();
    for i in 1..=2*n+1 {
        set.insert(i);
    }

    loop {
        let res = *set.iter().next().unwrap();
        println!("{}", res);
        input! {from &mut source, a: usize}

        if a == 0 {
            break;
        }

        set.remove(&res);
        set.remove(&a);
    }
}
