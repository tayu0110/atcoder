use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, c: usize, d: [[usize; n]; n]}

    // 0: only car, 1: use train
    let mut dist = vec![vec![usize::MAX; 2]; n];
    dist[0][0] = 0;

    let mut now = 0;
    let mut kind = 0;
    let mut reached = vec![vec![false; 2]; n];
    for _ in 0.. {
        reached[now][kind] = true;
        if kind == 0 {
            // still car
            for to in 0..n {
                if !reached[to][0] {
                    let cost = d[now][to] * a;
                    dist[to][0] = dist[to][0].min(dist[now][0] + cost);
                }

                // exchange to train
                if !reached[to][1] {
                    let cost = d[now][to] * b + c;
                    dist[to][1] = dist[to][1].min(dist[now][0] + cost);
                }
            }
        } else {
            // train
            for to in 0..n {
                if !reached[to][1] {
                    let cost = d[now][to] * b + c;
                    dist[to][1] = dist[to][1].min(dist[now][1] + cost);
                }
            }
        }

        let mut next = now;
        let mut next_kind = 0;
        let mut min = usize::MAX;
        for i in 0..n {
            for j in 0..2 {
                if !reached[i][j] && dist[i][j] < min {
                    min = dist[i][j];
                    next = i;
                    next_kind = j;
                }
            }
        }

        now = next;
        kind = next_kind;

        if min == usize::MAX {
            break;
        }
    }

    println!("{}", dist[n - 1][0].min(dist[n - 1][1]));
}
