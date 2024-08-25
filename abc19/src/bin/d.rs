use std::io::Write;

use proconio::*;

fn main() {
    input_interactive!(n: usize);

    let mut d = vec![0; n];
    for i in 1..n {
        println!("? 1 {}", i + 1);
        std::io::stdout().flush().unwrap();
        input_interactive!(t: usize);
        d[i] = t;
    }

    let &max = d.iter().max().unwrap();
    let root = d.iter().position(|p| *p == max).unwrap() + 1;
    let mut max = 0;
    for i in 1..=n {
        if i != root {
            println!("? {root} {i}");
            std::io::stdout().flush().unwrap();
            input_interactive!(t: usize);
            max = max.max(t);
        }
    }

    println!("! {max}");
    std::io::stdout().flush().unwrap();
}
