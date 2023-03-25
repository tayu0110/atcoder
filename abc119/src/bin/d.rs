#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize, q: usize, s: [usize; a], t: [usize; b], x: [usize; q]}

    let mut ns = vec![std::usize::MAX; a];
    let mut nt = vec![std::usize::MAX; b];

    for (i, &s) in s.iter().enumerate() {
        let (mut l, mut r) = (0, b);
        while r - l > 1 {
            let m = (r + l) / 2;
            if t[m] <= s {
                l = m;
            } else {
                r = m;
            }
        }

        ns[i] = std::cmp::max(s, t[l]) - std::cmp::min(s, t[l]);
        if r < b {
            ns[i] = std::cmp::min(ns[i], std::cmp::max(s, t[r]) - std::cmp::min(s, t[r]));
        }
    }
    for (i, &t) in t.iter().enumerate() {
        let (mut l, mut r) = (0, a);
        while r - l > 1 {
            let m = (r + l) / 2;
            if s[m] <= t {
                l = m;
            } else {
                r = m;
            }
        }

        nt[i] = std::cmp::max(t, s[l]) - std::cmp::min(t, s[l]);
        if r < a {
            nt[i] = std::cmp::min(nt[i], std::cmp::max(t, s[r]) - std::cmp::min(t, s[r]));
        }
    }

    for x in x {
        let mut res = std::usize::MAX;
        {
            let (mut l, mut r) = (0, a);
            while r - l > 1 {
                let m = (r + l) / 2;
                if s[m] <= x {
                    l = m;
                } else {
                    r = m;
                }
            }

            res = std::cmp::min(res, std::cmp::max(s[l], x) - std::cmp::min(s[l], x) + ns[l]);
            if r < a {
                res = std::cmp::min(res, std::cmp::max(s[r], x) - std::cmp::min(s[r], x) + ns[r]);
            }
        }
        {
            let (mut l, mut r) = (0, b);
            while r - l > 1 {
                let m = (r + l) / 2;
                if t[m] <= x {
                    l = m;
                } else {
                    r = m;
                }
            }

            res = std::cmp::min(res, std::cmp::max(t[l], x) - std::cmp::min(t[l], x) + nt[l]);
            if r < b {
                res = std::cmp::min(res, std::cmp::max(t[r], x) - std::cmp::min(t[r], x) + nt[r]);
            }
        }

        println!("{}", res);
    }
}
