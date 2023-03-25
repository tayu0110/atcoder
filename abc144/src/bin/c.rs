#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};

    let mut res = 0x3f3f3f3f3f3f3f3f;
    for i in 1..=n {
        if i * i > n {
            break;
        }
        if n % i != 0 {
            continue;
        }
        res = std::cmp::min(res, i + n / i - 2);
    }

    println!("{}", res);
}
