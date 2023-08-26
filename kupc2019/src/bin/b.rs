use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, w: usize, p: [(usize, usize); n], e: [(usize, usize); m]}

    let mut uf = UnionFind::new(n);
    for (a, b) in e {
        uf.merge(a - 1, b - 1);
    }

    let mut ws = vec![0; n];
    let mut vs = vec![0; n];
    for (i, (w, v)) in p.into_iter().enumerate() {
        let r = uf.root(i);
        ws[r] += w;
        vs[r] += v;
    }

    let mut dp = vec![0; w + 1];
    for (nw, nv) in ws.into_iter().zip(vs) {
        for i in (0..w).rev() {
            if i + nw > w {
                continue;
            }

            dp[i + nw] = dp[i + nw].max(dp[i] + nv);
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
