use proconio::*;

fn main() {
    input! {n: usize, k: usize, x: usize, y: usize}

    if n > k {
        println!("{}", k * x + (n - k) * y)
    } else {
        println!("{}", n * x);
    }
}
