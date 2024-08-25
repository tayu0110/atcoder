use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, k: usize, a: [i64; n]}

    let mut a = a.into_iter().filter(|&a| a != 0).collect::<Vec<_>>();
    if a.len() < k {
        println!("0");
        return;
    }

    a.sort_by_key(|a| -a.abs());

    let res = a[..k]
        .iter()
        .fold(Modint::one(), |s, v| s * Modint::from(*v));
    if a[..k].iter().filter(|&&a| a < 0).count() % 2 == 0 {
        println!("{}", res);
        return;
    }

    let mut p = [None; 2];
    let mut m = [None; 2];
    for &a in a.iter().take(k).rev() {
        if a > 0 && p[0].is_none() {
            p[0] = Some(a);
        } else if a < 0 && m[0].is_none() {
            m[0] = Some(a);
        }
    }
    for &a in a.iter().skip(k) {
        if a > 0 && p[1].is_none() {
            p[1] = Some(a);
        } else if a < 0 && m[1].is_none() {
            m[1] = Some(a);
        }
    }

    if let (Some(p0), Some(p1), Some(m0), Some(m1)) = (p[0], p[1], m[0], m[1]) {
        let (mul, div) = if p0 * p1 < m0 * m1 {
            (Modint::from(m1), Modint::from(p0))
        } else {
            (Modint::from(p1), Modint::from(m0))
        };
        println!("{}", res * mul / div);
    } else if let (Some(p0), Some(m1)) = (p[0], m[1]) {
        println!("{}", res * Modint::from(m1) / Modint::from(p0));
    } else if let (Some(p1), Some(m0)) = (p[1], m[0]) {
        println!("{}", res * Modint::from(p1) / Modint::from(m0));
    } else if p[0].is_none() && p[1].is_none() {
        if a.len() == n {
            println!(
                "{}",
                a.iter()
                    .rev()
                    .take(k)
                    .fold(Modint::one(), |s, v| s * Modint::from(*v))
            );
        } else {
            println!("0")
        }
    } else {
        println!("{}", res)
    }
}
