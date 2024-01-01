use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let n = n % k;
    println!("{}", n.min(k - n))
}
