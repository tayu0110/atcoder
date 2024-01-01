use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, i64, i64); m]}

    let mut t = vec![vec![]; n];
    for (u, v, b, c) in e {
        t[u - 1].push((v - 1, b, c));
    }

    let (mut l, mut r) = (0.0, 1e13);
    while r - l > 1e-10 {
        let m = (r + l) / 2.0;

        let mut dist = vec![f64::MIN; n];
        dist[0] = 0.0;
        for now in 0..n {
            for &(to, b, c) in &t[now] {
                let new = dist[now] + b as f64 - m * c as f64;
                if new > dist[to] {
                    dist[to] = new;
                }
            }
        }

        if dist[n - 1] > -1e-15 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{l:.20}");
}
