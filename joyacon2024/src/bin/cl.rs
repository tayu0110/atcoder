use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2)
}
