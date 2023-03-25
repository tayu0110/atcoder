#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let mut res = 0usize;
    for a in (1..=n).take_while(|v| v*v*v <= n) {
        for b in (a..=n).take_while(|v| a*v*v <= n) {
            let ab = a * b;
            let c_max = n / ab;
            if c_max >= b {
                res += c_max - b + 1;
            }
        }
    }

    println!("{}", res);
}
