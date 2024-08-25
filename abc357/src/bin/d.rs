use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn rec(n: usize, a: usize) -> (usize, usize) {
    if n == 1 {
        return (a % M, a.to_string().len());
    }

    let (r, mut len) = rec(n / 2, a);
    let p = 10usize.pow_mod(len as u64, M);
    len *= 2;
    let mut res = (r + (r * p % M)) % M;
    if n % 2 == 1 {
        let p2 = p * p % M;
        let na = p2 * (a % M) % M;
        res += na;
        res %= M;
        len += a.to_string().len();
    }

    (res, len)
}

fn main() {
    input! {n: usize}

    println!("{}", rec(n, n).0);
}
