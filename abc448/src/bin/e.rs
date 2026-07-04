use math::MathInt;
use proconio::*;
use rustc_hash::FxHashMap;

const M: usize = 10007;

fn rec(c: usize, l: usize, m: usize, memo: &mut [FxHashMap<usize, usize>]) -> usize {
    if let Some(ret) = memo[c].get(&l) {
        return *ret;
    }

    if l < 19 {
        let mut t = 0;
        for _ in 0..l {
            t *= 10;
            t += c;
        }

        t %= m;
        memo[c].insert(l, t);
        t
    } else {
        let half = l / 2;
        let lo = rec(c, half, m, memo);
        let hi = rec(c, l - half, m, memo);
        let ret = (hi * 10usize.pow_mod(half as u64, m) + lo) % m;
        memo[c].insert(l, ret);
        ret
    }
}

fn main() {
    input! {k: usize, m: usize, p: [(usize, usize); k]}

    let mut memo = vec![FxHashMap::default(); 10];
    let mut carry = 0;
    for &(c, l) in &p {
        carry = (rec(c, l, m, &mut memo) + 10usize.pow_mod(l as u64, m) * carry) % m;
    }

    let mut memo2 = vec![FxHashMap::default(); 10];
    let mut carry2 = 0;
    for &(c, l) in &p {
        carry2 = (rec(c, l, M, &mut memo2) + 10usize.pow_mod(l as u64, M) * carry2) % M;
    }

    println!(
        "{}",
        ((carry2 + M - carry % M) * m.inverse_mod(M).unwrap()) % M
    );
}
