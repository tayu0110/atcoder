use std::collections::BTreeSet;

use proconio::*;

fn solve(trc: &mut [i64; 2], arc: &mut [i64; 2], s: char, a: i64, mut t: char, b: i64) -> i64 {
    assert_eq!(a, b);
    let mut res = 0;
    if s == t {
        if trc == arc {
            res += a;
        }
        let (dr, dc) = match s {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        trc[0] += a * dr;
        arc[0] += a * dr;
        trc[1] += a * dc;
        arc[1] += a * dc;
        return res;
    }

    let (mut tr, mut tc) = (trc[0], trc[1]);
    let (mut ar, mut ac) = (arc[0], arc[1]);
    for (rc, s, a) in [(trc, s, a), (arc, t, b)] {
        match s {
            'U' => rc[0] -= a,
            'D' => rc[0] += a,
            'L' => rc[1] -= a,
            'R' => rc[1] += a,
            _ => {}
        }
    }

    if tr == ar && tc == ac {
        return 0;
    }

    let arr = ['R', 'U', 'L', 'D'];
    for (i, c) in arr.into_iter().enumerate() {
        if c == s {
            let pos = arr.iter().position(|&c| c == t).unwrap();
            t = arr[(pos + 4 - i) % 4];
            break;
        }
        (tr, tc) = (tc, -tr);
        (ar, ac) = (ac, -ar);
    }

    match t {
        'U' => {
            if ar > tr && ar - tr == ac - tc && ar - tr <= a {
                res += 1;
            }
        }
        'D' => {
            if ar < tr && tr - ar == ac - tc && tr - ar <= a {
                res += 1;
            }
        }
        'L' => {
            if ar == tr && ac > tc && (ac - tc) % 2 == 0 && ac - tc <= 2 * a {
                res += 1;
            }
        }
        _ => unreachable!(),
    }

    res
}

fn main() {
    input! {t: [i64; 2], a: [i64; 2], _: usize, m: usize, l: usize, mut qt: [(char, i64); m], mut qa: [(char, i64); l]}

    let mut term = BTreeSet::new();
    for q in [&qt, &qa] {
        let mut cum = 0;
        for &(_, a) in q {
            cum += a;
            term.insert(cum);
        }
        term.insert(cum);
    }

    let reconstruct = |mut qt: Vec<(char, i64)>| {
        let mut i = 0;
        let mut q = vec![];
        let mut prev = 0;
        for &(mut t) in &term {
            let p = t;
            t -= prev;
            prev = p;
            while t > 0 && i < qt.len() {
                let (s, a) = qt[i];
                if t < a {
                    q.push((s, t));
                    qt[i].1 -= t;
                    break;
                } else {
                    q.push((s, a));
                    t -= a;
                }
                i += 1;
            }
        }
        if i < qt.len() {
            q.push(qt[i]);
        }
        q
    };
    let qt = reconstruct(qt);
    let qa = reconstruct(qa);

    let mut trc = [t[0], t[1]];
    let mut arc = [a[0], a[1]];
    let mut res = 0i64;
    for ((s, a), (t, b)) in qt.into_iter().zip(qa) {
        assert_eq!(a, b);
        res += solve(&mut trc, &mut arc, s, a, t, b);
    }

    println!("{}", res);
}
