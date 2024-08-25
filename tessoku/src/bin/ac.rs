use proconio::*;

const MOD: u64 = 1_000_000_007;

fn pow(a: u64, mut b: u64) -> u64 {
    let (mut val, mut res) = (a, 1);
    while b > 0 {
        if b & 1 != 0 {
            res *= val;
            res %= MOD;
        }
        val *= val;
        val %= MOD;
        b >>= 1;
    }
    res
}

fn main() {
    input! {a: u64, b: u64}
    println!("{}", pow(a, b))
}
