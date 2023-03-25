#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, t: usize, s: [usize; n]}

    let mut res = 0;
    let mut now = 0;
    for v in s {
        if now > v {
            if now <= v + t {
                res += v + t - now;
                now = v + t;
            }
        } else {
            now = v + t;
            res += t;
        }
    }

    println!("{}", res);
}
