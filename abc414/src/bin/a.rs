use proconio::*;

fn main() {
    input! {n: usize, l: usize, r: usize, p: [(usize, usize); n]}

    println!(
        "{}",
        p.into_iter().filter(|&(x, y)| { x <= l && r <= y }).count()
    )
}
