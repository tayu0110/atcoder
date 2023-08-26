use proconio::*;

fn main() {
    input! {n: usize, p: [(i32, i32); n]}
    println!(
        "{}",
        p.windows(2).fold(0, |s, v| s
            + (v[1].0 - v[0].0).abs()
            + (v[1].1 - v[0].1).abs())
    )
}
