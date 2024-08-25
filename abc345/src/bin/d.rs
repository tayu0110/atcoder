use itertools::Itertools;
use proconio::*;

fn dfs(h: usize, w: usize, t: &[(usize, usize)], f: &mut [Vec<bool>]) -> bool {
    if t.is_empty() {
        return true;
    }

    let (r, c) = t[0];
    for i in 0..h {
        for j in 0..w {
            if i + r > h || j + c > w {
                continue;
            }

            if (i..i + r).any(|i| f[i][j..j + c].iter().any(|&b| b)) {
                continue;
            }

            for i in i..i + r {
                for j in j..j + c {
                    f[i][j] = true;
                }
            }

            return dfs(h, w, &t[1..], f);
        }
    }

    false
}

fn main() {
    input! {n: usize, h: usize, w: usize, p: [(usize, usize); n]}

    for i in 0..1 << n {
        let p = p
            .iter()
            .enumerate()
            .filter_map(|(j, v)| (i & (1 << j) != 0).then_some(v))
            .collect::<Vec<_>>();

        if p.iter().map(|(a, b)| a * b).sum::<usize>() != h * w {
            continue;
        }

        let n = p.len();
        for i in 0..1 << n {
            let p = p
                .iter()
                .enumerate()
                .map(|(j, v)| {
                    if i & (1 << j) != 0 {
                        (v.0, v.1)
                    } else {
                        (v.1, v.0)
                    }
                })
                .collect::<Vec<_>>();
            for v in (0..n).permutations(n) {
                let mut q = vec![(0, 0); n];
                for (i, v) in v.into_iter().enumerate() {
                    q[v] = p[i];
                }

                let mut f = vec![vec![false; w]; h];
                if dfs(h, w, &q, &mut f) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
