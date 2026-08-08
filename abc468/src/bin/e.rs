use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ret = 0;
    let mut raw = vec![0; n + 1];
    let mut mul = vec![0; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        let m = (i + 1).min(n - i);
        ret += m * a % M;
        ret %= M;

        raw[m] += m * a % M;
        raw[m] %= M;
        raw[n - m] += M - m * a % M;
        raw[n - m] %= M;

        mul[n - m] += a;
        mul[n - m] %= M;
    }
    // eprintln!("{ret}");

    for i in 0..n {
        raw[i + 1] += raw[i];
        raw[i + 1] %= M;
        mul[i + 1] += mul[i];
        mul[i + 1] %= M;
    }
    // eprintln!("{raw:?}, mul: {mul:?}");

    for i in 0..n {
        ret += raw[i] * (i + 1).inverse_mod(M).unwrap();
        ret %= M;
        ret += mul[i] * (n - i) % M * (i + 1).inverse_mod(M).unwrap();
        ret %= M;
    }

    println!("{ret}");
}
