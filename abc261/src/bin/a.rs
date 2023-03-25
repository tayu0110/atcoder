#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {l1: usize, r1: usize, l2: usize, r2: usize};
    let mut v = vec![0; 1010];
    for i in l1..=r1 {
        v[i] += 1;
    }
    for i in l2..=r2 {
        v[i] += 1;
    }
    let mut res = 0;
    for w in v {
        if w == 2 {
            res += 1;
        }
    }
    if res > 0 {
        res -= 1;
    }
    println!("{}", res);
}
