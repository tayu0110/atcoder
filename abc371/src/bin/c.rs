use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mg: usize, eg: [(usize, usize); mg], mh: usize, eh: [(usize, usize); mh]}

    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! {na: usize}
            a[i][j] = na;
        }
    }

    let mut g = vec![vec![false; n]; n];
    for (u, v) in eg {
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }

    let mut res = usize::MAX;
    for w in (0..n).permutations(n) {
        let mut h = vec![vec![false; n]; n];
        for &(u, v) in &eh {
            let tu = w[u - 1];
            let tv = w[v - 1];
            h[tu][tv] = true;
            h[tv][tu] = true;
        }

        let mut sum = 0;
        for i in 0..n {
            for j in i + 1..n {
                let ti = w[i];
                let tj = w[j];

                if g[ti][tj] != h[ti][tj] {
                    sum += a[i][j];
                }
            }
        }

        res = res.min(sum);
    }

    println!("{res}")
}
