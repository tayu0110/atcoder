use proconio::*;

const MOD: u64 = 1000_000_007;

fn main() {
    input! {a: u64, mut b: u64}

    let (mut res, mut val) = (1, a);
    while b > 0 {
        if b & 1 != 0 {
            res *= val;
            res %= MOD;
        }

        val *= val;
        val %= MOD;
        b >>= 1;
    }

    println!("{res}");
}
