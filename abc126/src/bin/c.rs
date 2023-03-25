#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize};

    let mut res = 0.0;
    for mut now in 1..=n {
        let mut tmp = 1.0 / n as f64;
        while now < k {
            now *= 2;
            tmp /= 2.0;
        }
        res += tmp;
    }

    println!("{}", res);
}
