#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: usize, t: usize, m: usize, p: [(usize, usize); m]};
    let mut g = vec![vec![]; s];
    for (u, v) in p {
        g[u-1].push(v-s-1);
    }
    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let mut h = vec![vec![INF; t]; t];
    for (k, v) in g.into_iter().enumerate() {
        let len = v.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }
                if h[v[i]][v[j]] != INF {
                    println!("{} {} {} {}", h[v[i]][v[j]], k+1, v[i]+s+1, v[j]+s+1);
                    std::process::exit(0);
                }
                h[v[i]][v[j]] = k + 1;
            }
        }
    }

    println!("-1");
}
