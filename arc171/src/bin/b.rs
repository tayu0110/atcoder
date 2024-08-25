use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use rand::{thread_rng, Rng};
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn naive(n: usize, a: Vec<usize>) -> Modint {
    let mut res = 0;
    for v in (1..=n).permutations(n) {
        let p = v.clone();
        let mut b = (1..=n).collect::<Vec<_>>();
        for i in 0..n {
            while b[i] < p[b[i] - 1] {
                b[i] = p[b[i] - 1];
            }
        }

        if b == a {
            res += 1;
        }
    }
    Modint::new(res as u64)
}

fn solve(n: usize, a: Vec<usize>) -> Modint {
    // if n < 10 {
    //     return naive(n, a);
    // }

    let mut most_left = vec![usize::MAX; n + 1];
    let mut most_right = vec![0; n + 1];
    let mut msl = VecDeque::new();
    msl.push_back(0);
    for (i, &a) in a.iter().enumerate() {
        most_right[a] = i + 1;

        if most_left[a] == usize::MAX {
            most_left[a] = i + 1;
            msl.push_back(i + 1);
        }
    }

    for i in 1..n + 1 {
        if most_right[i] > 0 && most_right[i] != i {
            return Modint::zero();
        }
    }

    let mut res = Modint::one();
    let msr = most_right
        .into_iter()
        .filter(|&m| m > 0)
        .collect::<HashSet<_>>();
    // eprintln!("msl: {msl:?}, msr: {msr:?}");
    for i in 0..n {
        if msr.contains(&(i + 1)) {
            let pos = msl.partition_point(|&m| m <= i + 1);
            res *= Modint::new(pos.saturating_sub(1) as u64);
            msl.pop_front();
        }
    }

    res
}

fn main() {
    // input! {n: usize, a: [usize; n]}
    // println!("{}", solve(n, a));
    let mut rng = thread_rng();

    for _ in 0..1000000 {
        let n = rng.gen_range(1..10);
        let a = (0..n).map(|_| rng.gen_range(1..=n)).collect::<Vec<_>>();

        eprintln!("n: {n}, a: {a:?}");
        let nres = naive(n, a.clone());
        let res = solve(n, a);

        assert_eq!(nres, res);
    }
}
