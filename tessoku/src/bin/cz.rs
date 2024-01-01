use proconio::*;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        a %= b;
        (a, b) = (b, a);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {a: u64, b: u64}
    println!("{}", lcm(a, b))
}
