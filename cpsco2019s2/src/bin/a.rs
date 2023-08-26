use proconio::*;

fn main() {
    input! {m: usize, n: usize}

    println!("{}", m - (m / n) * (n - 1))
}
