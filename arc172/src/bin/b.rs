use itertools::Itertools;
use montgomery_modint::Mod998244353;
use proconio::*;
use rustc_hash::FxHashSet;
use static_modint::StaticModint;

type Modint = StaticModint<Mod998244353>;

#[allow(unused)]
fn naive_sub(n: usize, k: usize, l: usize, perm: &mut Vec<usize>) -> usize {
    if perm.len() == n {
        let mut set = FxHashSet::default();
        for c in (0..n).combinations(k) {
            let mut buf = vec![];
            for c in c {
                buf.push(perm[c]);
            }
            if set.contains(&buf) {
                return 0;
            }
            set.insert(buf);
        }

        return 1;
    }

    let mut res = 0;
    for v in 0..l {
        perm.push(v);
        res += naive_sub(n, k, l, perm);
        perm.pop();
    }

    res
}

#[allow(unused)]
fn naive(n: usize, k: usize, l: usize) -> usize {
    let mut buf = vec![];
    naive_sub(n, k, l, &mut buf)
}

fn main() {
    input! {n: u32, k: u32, l: u32}

    if l + k <= n {
        println!("0");
        return;
    }

    // for n in 2..=6 {
    //     for k in 2..n {
    //         for l in 1..10 {
    //             println!("n: {n}, k: {k}, l: {l}, result: {}", naive(n, k, l));
    //         }
    //         println!()
    //     }
    //     println!()
    // }
    // println!("{}", naive(n, k, l));

    let mut perm = Modint::one();
    for l in (1..=l).rev().take((n - k + 1) as usize) {
        perm *= Modint::new(l as u64);
    }

    let t = Modint::new((l - (n - k)) as u64).pow(k as u64 - 1);
    println!("{}", perm * t)
}
