use proconio::*;

fn main() {
    input! {n: usize, x: usize}

    println!("{}", x * 540 + (n - x) * 525)
}
