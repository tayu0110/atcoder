#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

// This is not binary_search.....
fn binary_search(base: i64, a: &[i64]) -> (i64, i64) {
    let (mut l, mut r) = (0, a.len());
    let max = *a.last().unwrap();

    while r - l > 2 {
        let (p, q) = ((l*2+r) / 3, (l+r*2) / 3);

        let dp = ((a[p] - base) - (max - a[p])).abs();
        let dq = ((a[q] - base) - (max - a[q])).abs();

        if dp < dq {
            r = q;
        } else {
            l = p;
        }
    }

    let mut res = (0, std::i64::MAX);
    for d in l.saturating_sub(5)..std::cmp::min(r+5, a.len()) {
        let (l, r) = (a[d] - base, max - a[d]);
        let (l, r) = (std::cmp::min(l, r), std::cmp::max(l, r));
        if res.1 - res.0 > r - l {
            res = (l, r);
        }
    }

    res
}

fn main() {
    input! {n: usize, a: [i64; n]}

    let s = a.iter().scan(0, |s, v| {
        *s += *v;
        Some(*s)
    }).collect::<Vec<_>>();

    let mut res = std::i64::MAX;
    for i in 2..n-1 {
        let (p, q) = binary_search(0, &s[0..i]);
        let (r, s) = binary_search(s[i-1], &s[i..]);

        // eprintln!("p: {}, q: {}, r: {}, s: {}", p, q, r, s);

        res = std::cmp::min(res, std::cmp::max(q, s) - std::cmp::min(p, r));
    }

    println!("{}", res);
}
