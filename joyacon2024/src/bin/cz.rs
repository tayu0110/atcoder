use proconio::*;

fn main() {
    input! {h: usize, w: usize, n: usize}

    println!("{}", (n + h.max(w) - 1) / h.max(w))
}
