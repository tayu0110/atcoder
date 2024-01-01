use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: usize}

    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {nc: usize, mut na: [usize; k]}
        na.resize(5, p);
        c.push(nc);
        a.push(na);
    }

    let mut dp = vec![vec![vec![vec![vec![usize::MAX; p + 1]; p + 1]; p + 1]; p + 1]; p + 1];
    dp[0][0][0][0][0] = 0;
    for (c, a) in c.into_iter().zip(a) {
        let mut new = dp.clone();
        for i in 0..=p {
            for j in 0..=p {
                for k in 0..=p {
                    for l in 0..=p {
                        for m in 0..=p {
                            if dp[i][j][k][l][m] == usize::MAX {
                                continue;
                            }

                            let (ni, nj, nk, nl, nm) = (
                                p.min(i + a[0]),
                                p.min(j + a[1]),
                                p.min(k + a[2]),
                                p.min(l + a[3]),
                                p.min(m + a[4]),
                            );

                            new[ni][nj][nk][nl][nm] =
                                new[ni][nj][nk][nl][nm].min(dp[i][j][k][l][m] + c);
                        }
                    }
                }
            }
        }

        dp = new;
    }

    println!("{}", dp[p][p][p][p][p] as i64);
}
