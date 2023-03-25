use std::collections::HashMap;

use proconio::*;

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn ext_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut sp, mut sq, mut sr) = (0, -1, a);
    let (mut tp, mut tq, mut tr) = (1, 0, b);
    while tr > 0 {
        let c = a / b;
        sp -= c * tp;
        sq -= c * tq;
        sr -= c * tr;

        std::mem::swap(&mut sp, &mut tp);
        std::mem::swap(&mut sq, &mut tq);
        std::mem::swap(&mut sr, &mut tr);
    }

    (sp, sq, sr)
}

fn solve(p: &[(i32, i32, i32)]) -> HashMap<(i32, i32), i32> {
    let n = p.len();
    let mut res = HashMap::new();

    for i in 0..1 << n {
        let mut cost = 0;
        let mut ma = 0;
        let mut mb = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                cost += p[j].2;
                ma += p[j].0;
                mb += p[j].1;
            }
        }

        if ma == 0 && mb == 0 {
            res.insert((0, 0), 0);
            continue;
        }

        let g = gcd(ma, mb);
        ma /= g;
        mb /= g;

        let entry = res.entry((ma, mb)).or_insert(std::i32::MAX);
        let min = (*entry).min(cost);
        *entry = min;
    }

    res
}

fn main() {
    input! {n: usize, ma: i32, mb: i32, p: [(i32, i32, i32); n]}

    let l = solve(&p[..n / 2]);
    let r = solve(&p[n / 2..]);

    let mut res = std::i32::MAX;
    for (l, r) in vec![(&l, &r), (&r, &l)] {
        for (k, v) in l {
            let &(a, b) = k;
            if a == 0 && b == 0 {
                if let Some(nv) = r.get(&(ma, mb)) {
                    res = res.min(*v + *nv);
                }
                continue;
            }
            let c = ma * b - mb * a;
            let (p, q, s) = ext_gcd(mb, ma);

            if c == 0 {
                eprintln!("a: {}, b: {}, c: {}", a, b, c);
                res = res.min(*v);
                continue;
            }

            if c % s != 0 {
                continue;
            }

            let (p, q) = (c / s * p, c / s * q);
            let t = mb * p - ma * q;
            eprintln!("a: {}, b: {}, c: {}, p: {}, q: {}", a, b, c, p, q);
            if t != c {
                eprintln!("a: {}, b: {}, c: {}, t: {}", a, b, c, t);
                continue;
            }
            assert_eq!(t, c);
            if let Some(&nv) = r.get(&(p, q)) {
                res = res.min(*v + nv);
            }
        }
    }

    if res == std::i32::MAX {
        println!("-1")
    } else {
        println!("{}", res)
    }
}
