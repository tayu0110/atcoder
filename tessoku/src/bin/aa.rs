use proconio::*;

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    input! {a: u64, b: u64}

    println!("{}", gcd(a, b));
}
