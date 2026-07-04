use fenwick_tree::{AbelianGroup, FenwickTree};
use math::MathInt;
use proconio::*;

const M: usize = 998244353;

struct T;
impl AbelianGroup for T {
    type G = u32;
    fn e() -> Self::G {
        0
    }
    fn op(l: &Self::G, r: &Self::G) -> Self::G {
        l.wrapping_add(*r)
    }
    fn inv(g: &Self::G) -> Self::G {
        g.wrapping_neg()
    }
}

fn main() {
    input! {n: usize, k: usize, p: [usize; n]}

    let rev = {
        let mut ft = FenwickTree::<T>::new(n + 1);
        let mut res = 0usize;
        for &p in &p {
            res += ft.fold(p..) as usize;
            ft.set(p, 1);
        }
        res %= M;
        res
    };

    let mut ft = FenwickTree::<T>::new(n + 1);
    let mut sum = 0usize;
    for i in 0..k {
        sum += ft.fold(p[i]..) as usize;
        ft.set(p[i], 1);
    }
    sum %= M;

    let e = k * (k - 1) % M as usize * 4usize.inverse_mod(M as usize).unwrap() % M as usize;
    let mut res = rev + e + M - sum;
    res %= M;
    for i in k..n {
        ft.set(p[i - k], 0);
        sum += M - ft.fold(..p[i - k]) as usize;
        sum += ft.fold(p[i]..) as usize;
        sum %= M;
        ft.set(p[i], 1);
        res += rev + e + M - sum;
    }

    res %= M;
    res *= (n - k + 1).inverse_mod(M).unwrap();
    res %= M;
    println!("{res}")
}
