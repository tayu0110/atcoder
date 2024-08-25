use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {m: usize, k: usize};

    if k >= 1 << m {
        println!("-1");
        std::process::exit(0);
    }

    if m == 0 {
        println!("0 0");
        std::process::exit(0);
    }

    if m == 1 {
        if k == 0 {
            println!("0 0 1 1");
        } else {
            println!("-1");
        }
        std::process::exit(0);
    }

    let a = (0..(1 << m)).filter(|v| v != &k).collect_vec();
    let mut b = a.clone();
    b.reverse();

    for v in a {
        print!("{} ", v);
    }
    print!("{} ", k);
    for v in b {
        print!("{} ", v);
    }
    println!("{}", k);
}
