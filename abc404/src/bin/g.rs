use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]}

    let mut t = vec![vec![]; n + 1];
    for (l, r, s) in p {
        t[r].push((l - 1, -s));
        t[l - 1].push((r, s));
    }
    for i in 0..n {
        t[i + 1].push((i, -1));
    }

    let mut dist = vec![i64::MAX; n + 1];
    dist[n] = 0;
    let mut updated = false;
    for _ in 0..n + 1 {
        updated = false;
        for i in 0..n + 1 {
            for &(to, c) in &t[i] {
                if dist[i] < i64::MAX {
                    let next = dist[i].saturating_add(c);
                    if dist[to] > next {
                        dist[to] = next;
                        updated = true;
                    }
                }
            }
        }
    }

    if updated {
        println!("-1")
    } else {
        println!("{}", -dist[0]);
    }
}
