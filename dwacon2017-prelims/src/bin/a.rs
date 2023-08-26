use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}

    println!("{}", (a + b).saturating_sub(n))
}
