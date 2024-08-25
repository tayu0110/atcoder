use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut query: [(usize, usize, usize); q]}

    query.sort_unstable_by_key(|q| (q.2, -((q.1 - q.0) as i64)));
    query.iter_mut().for_each(|q| {
        q.0 -= 1;
        q.1 -= 1;
    });

    let mut res = 0;

    let mut ranges = BTreeSet::new();
    let mut used = vec![usize::MAX; n];
    let mut set = (0..n).collect::<BTreeSet<_>>();
    for (i, &(mut l, mut r, c)) in query.iter().enumerate() {
        let buf = set.range(l..=r).cloned().collect::<Vec<_>>();
        if buf.is_empty() {
            let &(pl, pr) = ranges.range(..=(l, usize::MAX)).next_back().unwrap();
            let mut buf = vec![];
            for &(pl, pr) in ranges.range((pl, pr)..) {
                if pl > r {
                    break;
                }
                res += c;
                l = l.min(pl);
                r = r.max(pr);
                buf.push((pl, pr));
            }

            for (l, r) in buf {
                ranges.remove(&(l, r));
            }
            ranges.insert((l, r));
            continue;
        }

        if buf[0] != l {
            let &(pl, pr) = ranges.range(..=(l, usize::MAX)).next_back().unwrap();
            let mut b = vec![];
            for &(l, r) in ranges.range((pl, pr)..) {
                if l > buf[0] {
                    break;
                }
                b.push((l, r));
                res += c;
            }
            l = b[0].0;
            for (l, r) in b {
                ranges.remove(&(l, r));
            }
        }

        if buf.last().unwrap() != &r {
            let &(pl, pr) = ranges.range(..=(r, usize::MAX)).next_back().unwrap();
            let mut b = vec![];
            for &(l, r) in ranges.range(..=(pl, pr)).rev() {
                if &r < buf.last().unwrap() {
                    break;
                }
                b.push((l, r));
                res += c;
            }
            r = b.last().unwrap().1;
            for (l, r) in b {
                ranges.remove(&(l, r));
            }
        }
        res += buf.len() * c;
        for &b in &buf {
            set.remove(&b);
            used[b] = i;
        }

        if buf.len() != r + 1 - l {
            for v in buf.windows(2) {
                if v[1] - v[0] != 1 {
                    let l = v[0] + 1;
                    let r = v[1] - 1;
                    let mut b = vec![];
                    for &(l, r) in ranges.range((l, 0)..).take_while(|p| p.1 <= r) {
                        b.push((l, r));
                        res += c;
                    }
                    for (l, r) in b {
                        ranges.remove(&(l, r));
                    }
                }
            }
        }

        ranges.insert((l, r));
    }

    if used.iter().any(|&u| u == usize::MAX) || ranges.len() != 1 {
        println!("-1");
        return;
    }

    println!("{}", res);
}
