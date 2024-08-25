use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut t = ((k - 1) * (n - k) * 6 + (n - k) * 3 + (k - 1) * 3 + 1) as f64;
    t /= n as f64;
    t /= n as f64;
    t /= n as f64;
    println!("{}", t);
}
