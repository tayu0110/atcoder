use itertools::Itertools;
use proconio::*;

fn rec(now: usize, lo: i64, hi: i64, p: &[(i64, i64)], res: &mut Vec<i64>) -> i64 {
    if now == p.len() {
        return 0;
    }

    let (l, h) = p[now];
    let t = rec(now + 1, lo + l, hi + h, p, res);
    let (lo, hi) = (-hi, -lo);
    let mut r = l;
    if t + l < lo {
        let d = lo - (t + l);
        r = l + d;
        assert!(r <= h);
    }

    assert!(lo <= t + r && t + r <= hi);
    res.push(r);
    t + r
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    if p.iter().map(|p| p.0).sum::<i64>() > 0 || p.iter().map(|p| p.1).sum::<i64>() < 0 {
        println!("No");
        return;
    }

    let mut res = vec![];
    rec(0, 0, 0, &p, &mut res);
    res.reverse();
    println!("Yes");
    println!("{}", res.iter().join(" "))
}
