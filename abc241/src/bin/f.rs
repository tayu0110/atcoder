use proconio::*;
use std::collections::{HashMap, VecDeque};

fn bin_search(tar: usize, v: &[usize]) -> usize {
    let (mut l, mut r) = (0, v.len());
    while r - l > 1 {
        let m = (r + l) / 2;
        if tar < v[m] {
            r = m;
        } else {
            l = m;
        }
    }
    r
}

fn main() {
    input! {h: usize, w: usize, n: usize, sx: usize, sy: usize, gx: usize, gy: usize, mut p: [(usize, usize); n]}

    let mut row = HashMap::new();
    let mut col = HashMap::new();
    let mut point = HashMap::new();
    for (i, &(x, y)) in p.iter().enumerate() {
        row.entry(x).or_insert(vec![0, w + 1]).push(y);
        col.entry(y).or_insert(vec![0, h + 1]).push(x);
        point.insert((x, y), i);
    }
    p.push((sx, sy));
    for v in row.values_mut() {
        v.sort();
    }
    for v in col.values_mut() {
        v.sort();
    }

    // [up, down, left, right], n = s
    let mut dist = vec![vec![usize::MAX; 5]; n + 1];
    let mut nt = VecDeque::new();
    nt.push_back((0, 4, n));
    while let Some((nd, d, now)) = nt.pop_front() {
        if dist[now][d] < usize::MAX {
            continue;
        }
        dist[now][d] = nd;
        let (mut r, mut c) = p[now];
        if d == 0 {
            r -= 1;
        } else if d == 1 {
            r += 1;
        } else if d == 2 {
            c -= 1;
        } else if d == 3 {
            c += 1;
        }
        if let Some(v) = row.get(&r) {
            let upper = bin_search(c, &v);
            if upper < v.len() - 1 {
                let next = *point.get(&(r, v[upper])).unwrap();
                if dist[next][2] == usize::MAX {
                    nt.push_back((nd + 1, 2, next));
                }
            }
            if c > v[upper - 1] {
                if upper > 1 {
                    let next = *point.get(&(r, v[upper - 1])).unwrap();
                    if dist[next][3] == usize::MAX {
                        nt.push_back((nd + 1, 3, next));
                    }
                }
            }
        }
        if let Some(v) = col.get(&c) {
            let upper = bin_search(r, v);
            if upper < v.len() - 1 {
                let next = *point.get(&(v[upper], c)).unwrap();
                if dist[next][0] == usize::MAX {
                    nt.push_back((nd + 1, 0, next));
                }
            }
            if r > v[upper - 1] {
                if upper > 1 {
                    let next = *point.get(&(v[upper - 1], c)).unwrap();
                    if dist[next][1] == usize::MAX {
                        nt.push_back((nd + 1, 1, next));
                    }
                }
            }
        }
    }

    let mut res = usize::MAX;
    for (i, (x, y)) in p.into_iter().enumerate() {
        for (j, (dx, dy)) in vec![(!0, 0), (1, 0), (0, !0), (0, 1)]
            .into_iter()
            .enumerate()
        {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if gx == nx && gy == ny {
                res = res.min(dist[i][j]);
            }
        }
    }

    println!("{}", res as i64)
}
