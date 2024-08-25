use proconio::*;

const MAX: usize = 200001;
const M: u32 = 1_000_000_007;

const fn pow(a: u32, mut exp: u32) -> u32 {
    let (mut res, mut val) = (1, a as u64);
    while exp > 0 {
        if exp & 1 != 0 {
            res *= val;
            res %= M as u64;
        }
        val *= val;
        val %= M as u64;
        exp >>= 1;
    }
    res as u32
}

const FRAC: [u32; MAX] = {
    let mut buf = [0; MAX];
    buf[0] = 1;
    let mut i = 1;
    while i < MAX {
        buf[i] = (buf[i - 1] as u64 * i as u64 % M as u64) as u32;
        i += 1;
    }
    buf
};
const IFRAC: [u32; MAX] = {
    let mut buf = [0; MAX];
    buf[MAX - 1] = pow(FRAC[MAX - 1], M - 2);
    let mut i = MAX - 1;
    while i > 0 {
        i -= 1;
        buf[i] = (buf[i + 1] as u64 * (i + 1) as u64 % M as u64) as u32;
    }
    buf
};

const fn binom(n: usize, m: usize) -> u32 {
    (FRAC[n] as u64 * IFRAC[m] as u64 % M as u64 * IFRAC[n - m] as u64 % M as u64) as u32
}

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut res = 0u32;
    for (i, a) in a.into_iter().enumerate() {
        res += (binom(n - 1, i) as u64 * a as u64 % M as u64) as u32;
        res -= M & ((res >= M) as u32).wrapping_neg();
    }

    println!("{res}")
}
