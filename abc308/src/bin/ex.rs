use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, u32); m]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut res = std::u32::MAX;
    for i in 0..n {
        let len = t[i].len();
        for j in 0..len {
            let (j, c1) = t[i][j];
            if c1 > res {
                continue;
            }
            for k in 0..len {
                let (k, c2) = t[i][k];
                if j == k {
                    continue;
                }

                let rem = c1 + c2;
                if rem > res {
                    continue;
                }

                let mut reached = vec![false; n];
                let mut dist = vec![std::u32::MAX; n];
                dist[i] = 0;
                let mut now = i;
                for _ in 0..n {
                    if reached[now] {
                        break;
                    }
                    reached[now] = true;
                    if now == j {
                        break;
                    }
                    for &(to, c) in &t[now] {
                        if to == k {
                            continue;
                        }
                        if now == i && to == j {
                            continue;
                        }
                        if dist[now] + c + rem >= res {
                            continue;
                        }
                        dist[to] = dist[to].min(dist[now] + c);
                    }

                    let mut max = std::u32::MAX;
                    for i in (0..n).filter(|&i| !reached[i]) {
                        if dist[i] < max {
                            now = i;
                            max = dist[i];
                        }
                    }
                }

                if !reached[j] {
                    continue;
                }

                res = res.min(dist[j] + rem);
            }
        }
    }

    if res == std::u32::MAX {
        println!("-1")
    } else {
        println!("{}", res);
    }
}
