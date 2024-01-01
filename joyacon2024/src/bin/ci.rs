use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}

    println!("{}", n / (a + b) * a + (n % (a + b)).min(a))
}
