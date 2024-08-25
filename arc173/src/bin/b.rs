use std::collections::BinaryHeap;

use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a = a.rem_euclid(b);
        (a, b) = (b, a);
    }
    a.abs()
}

fn line(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64, i64) {
    let a = y2 - y1;
    let b = x2 - x1;
    let c = b * y1 - a * x1;
    let sgn = a.signum();
    if a == 0 && b == 0 {
        (0, 0, 0)
    } else if a == 0 {
        let sgn = b.signum();
        let g = gcd(b, c);
        (0, b / g * sgn, c / g * sgn)
    } else if b == 0 {
        let g = gcd(a, c);
        (a / g * sgn, 0, c / g * sgn)
    } else if c == 0 {
        let g = gcd(a, b);
        (a / g * sgn, b / g * sgn, 0)
    } else {
        let g = gcd(a, gcd(b, c));
        (a / g * sgn, b / g * sgn, c / g * sgn)
    }
}

fn solve(sets: FxHashMap<(i64, i64, i64), FxHashSet<usize>>) -> usize {
    let mut sets = sets.values().cloned().collect::<Vec<_>>();
    let mut res = 0;
    while sets.len() > 1 {
        sets.sort_unstable_by_key(|s| s.len());

        let mut s0 = sets.pop().unwrap();
        let mut s1 = sets.pop().unwrap();

        if let Some(&dup) = s0.intersection(&s1).next() {
            if s0.len() > s1.len() {
                s0.remove(&dup);
            } else {
                s1.remove(&dup);
            }
        }

        while s0.len().max(s1.len()) > 1 {
            let (s0, s1) = if s0.len() > s1.len() {
                (&mut s0, &mut s1)
            } else {
                (&mut s1, &mut s0)
            };

            let &a = s0.iter().next().unwrap();
            let &b = s0.iter().next().unwrap();
            let &c = s1.iter().next().unwrap();
            s0.remove(&a);
            s0.remove(&b);
            s1.remove(&c);
            for i in 0..sets.len() {
                sets[i].remove(&a);
                sets[i].remove(&b);
                sets[i].remove(&c);
            }
            res += 1;
        }

        if s0.len() > 0 {
            sets.push(s0);
        }
        if s1.len() > 0 {
            sets.push(s1);
        }
    }

    res
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut line_set = FxHashMap::default();
    for i in 0..n {
        let (x1, y1) = p[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x2, y2) = p[j];
            let (a, b, c) = line(x1, y1, x2, y2);
            line_set
                .entry((a, b, c))
                .or_insert(FxHashSet::default())
                .insert(i);
        }
    }

    let sres = solve(line_set.clone());

    {
        let keys = line_set.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            if line_set.get(&key).unwrap().len() <= 2 {
                line_set.remove(&key);
            }
        }
    }

    let mut belong = vec![0; n];
    for i in 0..n {
        for set in line_set.values() {
            if set.contains(&i) {
                belong[i] += 1;
            }
        }
    }

    let mut nt = BinaryHeap::new();
    for i in 0..n {
        nt.push((belong[i], i));
    }
    let mut res = 0;
    let mut used = vec![false; n];
    'b: while let Some((bel, a)) = nt.pop() {
        if belong[a] != bel {
            continue;
        }

        if used[a] {
            continue;
        }

        let mut buf = vec![];
        while let Some((bbel, b)) = nt.pop() {
            if belong[b] != bbel {
                continue;
            }

            if used[b] {
                continue;
            }

            let mut ibuf = vec![];
            while let Some((cbel, c)) = nt.pop() {
                if belong[c] != cbel {
                    continue;
                }

                if used[c] {
                    continue;
                }

                if line_set.values().any(|set| {
                    let ab = set.contains(&a);
                    let bb = set.contains(&b);
                    let cb = set.contains(&c);
                    ab && bb && cb
                }) {
                    ibuf.push((cbel, c));
                    continue;
                }

                res += 1;

                let mut remove = vec![];
                for (key, value) in line_set.iter_mut() {
                    value.remove(&a);
                    value.remove(&b);
                    value.remove(&c);

                    if value.len() < 3 {
                        for &s in value.iter() {
                            belong[s] -= 1;
                            if !used[s] {
                                nt.push((belong[s], s));
                            }
                        }
                        remove.push(*key);
                    }
                }
                for key in remove {
                    line_set.remove(&key);
                }
                used[a] = true;
                used[b] = true;
                used[c] = true;

                nt.extend(ibuf);
                nt.extend(buf);
                continue 'b;
            }

            buf.push((bbel, b));
        }

        nt.extend(buf);
    }

    println!("{}", res.max(sres));
}
