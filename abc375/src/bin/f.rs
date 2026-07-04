use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, q: usize, e: [(usize, usize, usize); m]}

    let mut dist = vec![vec![usize::MAX; n]; n];
    for &(a, b, c) in &e {
        dist[a - 1][b - 1] = dist[a - 1][b - 1].min(c);
        dist[b - 1][a - 1] = dist[b - 1][a - 1].min(c);
    }

    let mut query = vec![];
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {i: usize}
            let (a, b, c) = e[i - 1];
            dist[a - 1][b - 1] = usize::MAX;
            dist[b - 1][a - 1] = usize::MAX;
            query.push((ty, a - 1, b - 1, c));
        } else {
            input! {x: usize, y: usize}
            query.push((ty, x - 1, y - 1, 0));
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }

    let mut res = vec![];
    while let Some((ty, a, b, c)) = query.pop() {
        if ty == 1 {
            dist[a][b] = dist[a][b].min(c);
            dist[b][a] = dist[b][a].min(c);

            for k in [a, b] {
                for i in 0..n {
                    for j in 0..n {
                        dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
                    }
                }
            }
        } else {
            res.push(dist[a][b] as i64);
        }
    }
    res.reverse();

    println!("{}", res.iter().join("\n"))
}
