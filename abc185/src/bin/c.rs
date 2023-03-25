#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut l: usize}
    l -= 1;

    let mut res = 1;
    let mut d = 1;
    for _ in 0..11 {
        res *= l;
        l -= 1;
        while d < 12 && res % d == 0 {
            res /= d;
            d += 1;
        }
    }

    println!("{}", res);
}
