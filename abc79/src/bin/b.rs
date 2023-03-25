#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let mut l = [0usize; 100];

    l[0] = 2;
    l[1] = 1;

    for i in 2..=n {
        l[i] = l[i-1] + l[i-2];
    }

    println!("{}", l[n]);
}
