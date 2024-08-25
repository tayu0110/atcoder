use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut a: [marker::Bytes; h], n: usize, mut med: [(usize, usize, usize); n]}
    med.iter_mut().for_each(|v| {
        v.0 -= 1;
        v.1 -= 1;
    });
    med.insert(0, (0, 0, 0));
    med.push((0, 0, 0));

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == b'S' {
                med[0] = (i, j, 0);
                a[i][j] = b'.';
            } else if a[i][j] == b'T' {
                *med.last_mut().unwrap() = (i, j, 0);
                a[i][j] = b'.';
            }
        }
    }
    // eprintln!("med: {med:?}");

    let mut d = vec![vec![]; n + 2];
    for i in 0..n + 2 {
        let (r, c, e) = med[i];
        let mut nt = VecDeque::new();
        nt.push_back((r, c));
        let mut dist = vec![vec![usize::MAX; w]; h];
        dist[r][c] = 0;
        while let Some((r, c)) = nt.pop_front() {
            for (dy, dx) in vec![(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let nr = r.wrapping_add(dy);
                let nc = c.wrapping_add(dx);
                if nr < h && nc < w && a[nr][nc] == b'.' && dist[nr][nc] == usize::MAX {
                    dist[nr][nc] = dist[r][c] + 1;
                    nt.push_back((nr, nc));
                }
            }
        }

        for j in 0..n + 2 {
            let (nr, nc, _) = med[j];
            if dist[nr][nc] <= e {
                d[i].push(j);
            }
        }
    }

    let mut nt = VecDeque::new();
    nt.push_back(0);
    let mut used = vec![false; n + 2];
    used[0] = true;
    while let Some(now) = nt.pop_front() {
        for &to in &d[now] {
            if to == n + 1 {
                println!("Yes");
                return;
            }

            if !used[to] {
                used[to] = true;
                nt.push_back(to);
            }
        }
    }

    println!("No")
}
