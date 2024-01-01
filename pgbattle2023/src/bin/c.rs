use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn solve(mut t: Vec<usize>, com: &impl Fn(u32, u32) -> Modint) -> Modint {
    let mut len = t.len() as u32;
    t.sort();
    let mut a = vec![];
    for t in t {
        match a.last_mut() {
            Some((p, cnt)) if *p == t => *cnt += 1,
            _ => a.push((t, 1)),
        }
    }

    let mut res = Modint::one();
    for (_, cnt) in a {
        res *= com(len, cnt);
        len -= cnt;
    }

    res
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    if k == 0 {
        println!("1");
        return;
    }

    let mut t = vec![vec![]; k];
    for (i, a) in a.into_iter().enumerate() {
        t[i % k].push(a);
    }

    let mut res = Modint::one();
    let com = combination(n as u32 + 10);
    while let Some(t) = t.pop() {
        res *= solve(t, &com);
    }

    println!("{res}")
}
