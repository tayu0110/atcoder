#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: i64, a: i64, b: i64};

    if n < a {
        println!("0");
        std::process::exit(0);
    }

    let mut n = n - a;
    let rem = n % a;
    let mut res = 0;
    res += std::cmp::min(b, rem + 1);

    n -= rem;

    if a <= b {
        res += n;
    } else {
        res += n / a * b;
    }

    println!("{}", res);
}
