use proconio::*;

fn main() {
    input! {p: usize, q: usize, a: usize, b: usize}
    println!("{}", p.min(q) * a + q.saturating_sub(p) * b)
}
