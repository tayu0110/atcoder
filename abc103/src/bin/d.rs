#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m]}
    
    let mut d = vec![0; n+1];
    for (a, b) in p {
        d[b] = std::cmp::max(d[b], a);
    }

    let mut f = 1;
    let mut res = 0;
    for (i, v) in d.into_iter().enumerate().skip(1) {
        if v == 0 {
            continue;
        }
        if f <= v {
            res += 1;
            f = i;
        }
    }

    println!("{}", res);
}
