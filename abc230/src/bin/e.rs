#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let mut res = 0usize;
    let mut now = 1;
    while now <= n {
        let d = n / now;
        let k = n / d;
        res += (k - now + 1) * d;
        now = k + 1;
    }

    println!("{}", res);
}
