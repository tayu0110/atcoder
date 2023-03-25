#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut h: i64, p: [(i64, i64); n]}

    let (a, mut b) = p.into_iter().unzip::<i64, i64, Vec<_>, Vec<_>>();
    b.sort();
    let max = a.into_iter().max().unwrap();

    let mut res = 0;
    while let Some(now) = b.pop() {
        if max > now {
            break;
        }
        h -= now;
        res += 1;

        if h <= 0 {
            println!("{}", res);
            std::process::exit(0);
        }
    }

    println!("{}", res + (h + max - 1) / max);
}
