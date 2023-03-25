use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); n]}

    let mut p = p.into_iter().map(|(t, d)| (d, t)).collect::<Vec<_>>();
    p.sort_by_key(|&p| Reverse(p));

    let mut tp = vec![0; n];
    let mut v = vec![];
    let mut rem = vec![];
    for (d, t) in p {
        if tp[t - 1] > 0 {
            rem.push((d, t - 1));
        } else {
            tp[t - 1] += 1;
            v.push((d, t - 1));
        }
    }

    let mut kind = k;
    let mut res = k * k;
    if v.len() < k {
        kind = v.len();
        res = kind * kind;
        let rest = k - v.len();
        for &(_, t) in &rem[..rest] {
            tp[t] += 1;
        }
        v.extend(&rem[..rest]);
        rem = rem[rest..].to_vec();
    }

    while v.len() > k {
        let nv = v.pop().unwrap();
        tp[nv.1] -= 1;
        rem.push(nv);
    }

    v.sort();
    rem.sort();
    v.reverse();

    res += v.iter().map(|&(d, _)| d).sum::<usize>();

    let mut now = res;
    while let Some((d, t)) = rem.pop() {
        if v.is_empty() {
            break;
        }
        let (nd, nt) = v.pop().unwrap();

        if nt == t {
            break;
        }

        now += d;
        now -= nd;

        if tp[t] == 0 {
            now += (kind + 1) * (kind + 1);
            now -= kind * kind;
            kind += 1;
        }

        if tp[nt] == 1 {
            now += (kind - 1) * (kind - 1);
            now -= kind * kind;
            kind -= 1;
        }
        tp[t] += 1;
        tp[nt] -= 1;
        res = res.max(now);
    }

    println!("{}", res);
}
