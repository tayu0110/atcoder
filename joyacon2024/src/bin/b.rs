use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    println!("{}", s.into_iter().zip(t).filter(|(s, t)| s != t).count())
}
