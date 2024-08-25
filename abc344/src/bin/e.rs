use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n], q: usize}

    let mut map = BTreeMap::new();
    for &a in &a {
        map.insert(a, 0);
    }

    let mut query = vec![];
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, y: usize}
            map.insert(x, 0);
            map.insert(y, 0);
            query.push((t, x, y));
        } else {
            input! {x: usize}
            query.push((t, x, 0));
        }
    }

    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }
    let mut rev = vec![0; cnt];
    for (k, v) in &map {
        rev[*v] = *k;
    }

    a.iter_mut().for_each(|a| *a = *map.get(&a).unwrap());
    query.iter_mut().for_each(|v| {
        v.1 = *map.get(&v.1).unwrap();
        if v.0 == 1 {
            v.2 = *map.get(&v.2).unwrap();
        }
    });

    let mut prev = vec![None; cnt];
    let mut next = vec![None; cnt];
    for v in a.windows(2) {
        prev[v[1]] = Some(v[0]);
        next[v[0]] = Some(v[1]);
    }
    prev[a[0]] = Some(usize::MAX);
    next[*a.last().unwrap()] = Some(usize::MAX);

    for (t, x, y) in query {
        if t == 1 {
            let nt = next[x].unwrap();
            next[x] = Some(y);
            if nt < usize::MAX {
                prev[nt] = Some(y);
            }

            next[y] = Some(nt);
            prev[y] = Some(x);
        } else {
            let pr = prev[x].unwrap();
            let nt = next[x].unwrap();

            prev[x] = None;
            next[x] = None;

            if nt < usize::MAX {
                prev[nt] = Some(pr);
            }
            if pr < usize::MAX {
                next[pr] = Some(nt);
            }
        }
    }

    for i in 0..cnt {
        if prev[i].is_some() {
            let mut now = i;
            while let Some(prev) = prev[now] {
                if prev == usize::MAX {
                    break;
                }
                now = prev;
            }

            let mut res = vec![now];
            while let Some(next) = next[now] {
                if next == usize::MAX {
                    break;
                }
                res.push(next);
                now = next;
            }

            println!("{}", res.iter().map(|r| rev[*r]).join(" "));
            return;
        }
    }
}
