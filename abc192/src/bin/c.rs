#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: Chars, k: usize};

    for _ in 0..k {
        n.sort();
        let g2 = n.iter().collect::<String>().parse::<usize>().unwrap();
        n.reverse();
        let g1 = n.iter().collect::<String>().parse::<usize>().unwrap();
        let f = g1 - g2;
        n = f.to_string().chars().collect();
    }

    println!("{}", n.into_iter().collect::<String>().parse::<usize>().unwrap())
}
