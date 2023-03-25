#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {q: usize, a: i64, b: i64, p: [(i64, i64, i64); q]}

    let mut upper_apb = std::collections::BTreeSet::new();
    let mut lower_amb = std::collections::BTreeSet::new();
    let mut lower_apb = std::collections::BTreeSet::new();
    let mut upper_amb = std::collections::BTreeSet::new();

    upper_apb.insert(a + b);
    lower_amb.insert(a - b);
    lower_apb.insert(a + b);
    upper_amb.insert(a - b);

    for (t, a, b) in p {
        if t == 1 {
            upper_apb.insert(a + b);
            lower_amb.insert(a - b);
            lower_apb.insert(a + b);
            upper_amb.insert(a - b);
        } else {
            let mut res = std::i64::MAX;
            if let Some(&r) = upper_apb.range(..=b).next_back() {
                if a <= r {
                    res = 0;
                } else {
                    res = std::cmp::min(res, a - r);
                }
            }
            if let Some(&r) = upper_amb.range(..=b).next_back() {
                if a <= r {
                    res = 0;
                } else {
                    res = std::cmp::min(res, a - r);
                }
            }
            if let Some(&r) = lower_amb.range(a..).next() {
                if r <= b {
                    res = 0;
                } else {
                    res = std::cmp::min(res, r - b);
                }
            }
            if let Some(&r) = lower_apb.range(a..).next() {
                if r <= b {
                    res = 0;
                } else {
                    res = std::cmp::min(res, r - b);
                }
            }

            println!("{}", res);
        }
    }
}
