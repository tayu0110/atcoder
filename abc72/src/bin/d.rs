#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut p: [usize; n]}

    let mut res = 0;
    for i in 0..n-1 {
        if p[i] == i+1 {
            p.swap(i, i+1);
            res += 1;
        }
    }

    if p[n-1] == n {
        res += 1;
        p.swap(n-1, n-2);
    }

    println!("{}", res);
}
