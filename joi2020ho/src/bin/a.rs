use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n + 1], mut b: [usize; n]}
    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    a.sort_unstable_by_key(|a| a.1);
    b.sort_unstable();

    let mut cum = vec![0; n + 1];
    for (i, (&(_, a), &b)) in a.iter().zip(b.iter()).enumerate() {
        cum[i + 1] = a.saturating_sub(b).max(cum[i]);
    }

    let mut rcum = vec![0; n + 1];
    for (i, (&(_, a), &b)) in a.iter().rev().zip(b.iter().rev()).enumerate() {
        rcum[n - 1 - i] = a.saturating_sub(b).max(rcum[n - i]);
    }

    let mut res = vec![0; n + 1];
    for (c, (r, (i, _))) in cum.into_iter().zip(rcum.into_iter().zip(a)) {
        res[i] = c.max(r);
    }
    println!("{}", res.iter().join(" "))
}
