use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}
    println!(
        "{}",
        p.into_iter()
            .fold(0, |s, (a, b)| s + b * (b + 1) / 2 - a * (a - 1) / 2)
    );
}
