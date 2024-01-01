use itertools::Itertools;
use proconio::*;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

const MAX_ITER: usize = 100000;
const M1: usize = !0;

fn shortest_path(
    n: usize,
    now: usize,
    to: usize,
    reached: &mut Vec<Vec<bool>>,
    t: &Vec<Vec<usize>>,
) -> Vec<u8> {
    let mut nt = VecDeque::new();
    nt.push_back(now);
    let mut dist = vec![usize::MAX; n * n];
    dist[now] = 0;
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[to] == usize::MAX {
                dist[to] = dist[now] + 1 + reached[to / n][to % n] as usize;
                nt.push_back(to);
            }
        }
    }

    let mut res = vec![];
    nt.push_back(to);
    while let Some(now) = nt.pop_front() {
        let (r, c) = (now / n, now % n);
        let w = reached[r][c] as usize;
        reached[r][c] = true;
        if dist[now] == 0 {
            break;
        }

        for &to in &t[now] {
            if dist[to] + 1 + w == dist[now] {
                let (nr, nc) = (to / n, to % n);
                match (r.wrapping_sub(nr), c.wrapping_sub(nc)) {
                    (1, 0) => res.push(b'D'),
                    (0, 1) => res.push(b'R'),
                    (M1, 0) => res.push(b'U'),
                    (0, M1) => res.push(b'L'),
                    _ => unreachable!(),
                }

                nt.push_back(to);
                break;
            }
        }
    }

    res.reverse();
    res
}

fn main() {
    input! {n: usize, h: [marker::Bytes; n - 1], v: [marker::Bytes; n], _: [[usize; n]; n]}

    let mut t = vec![vec![]; n * n];
    for i in 0..n {
        for j in 0..n {
            if i < n - 1 && h[i][j] == b'0' {
                t[i * n + j].push((i + 1) * n + j);
                t[(i + 1) * n + j].push(i * n + j);
            }
            if j < n - 1 && v[i][j] == b'0' {
                t[i * n + j].push(i * n + j + 1);
                t[i * n + j + 1].push(i * n + j);
            }
        }
    }

    let mut rng = thread_rng();
    let mut rem = MAX_ITER;
    let mut prev = 0;
    let mut prev_len = 0;
    let mut now = 0;
    let mut reached = vec![vec![false; n]; n];
    reached[0][0] = true;
    let mut res = vec![];

    for r in 0..n {
        if r % 2 == 0 {
            for c in 0..n {
                if reached[r][c] {
                    continue;
                }
                let to = r * n + c;
                let path = shortest_path(n, now, to, &mut reached, &t);

                prev = now;
                now = to;
                rem -= path.len();
                prev_len = res.len();
                res.extend(path);
            }
        } else {
            for c in (0..n).rev() {
                if reached[r][c] {
                    continue;
                }
                let to = r * n + c;
                let path = shortest_path(n, now, to, &mut reached, &t);

                prev = now;
                now = to;
                rem -= path.len();
                prev_len = res.len();
                res.extend(path);
            }
        }
    }

    while rem > 0 {
        let retpath = shortest_path(n, now, 0, &mut reached, &t);
        if retpath.len() > rem {
            let path = shortest_path(n, prev, 0, &mut reached, &t);
            res.truncate(prev_len);
            res.extend(path);
            break;
        }

        let mut to = rng.gen_range(0..n * n);
        let mut path = shortest_path(n, now, to, &mut reached, &t);
        for _ in 0..100 {
            if rem > path.len() {
                break;
            }
            to = rng.gen_range(0..n * n);
            path = shortest_path(n, now, to, &mut reached, &t);
        }

        if rem <= path.len() {
            res.extend(retpath);
            break;
        }

        prev = now;
        now = to;
        rem -= path.len();
        prev_len = res.len();
        res.extend(path);
    }

    println!("{}", res.into_iter().map(|c| c as char).join(""))
}
