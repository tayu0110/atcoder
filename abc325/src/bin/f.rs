use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {n: usize, d: [usize; n], l1: usize, c1: usize, k1: usize, l2: usize, c2: usize, k2: usize}

    let mut dp = vec![vec![0; k2 + 1]; k1 + 1];

    let mut nt = vec![(0, 0)];
    for (i, d) in d.into_iter().enumerate() {
        let mut used = BTreeSet::new();

        for x in 0..=k1 {
            if (d + l1 - 1) / l1 * l1 < x * l1 {
                break;
            }
            let rem = d.saturating_sub(x * l1);
            let y = (rem + l2 - 1) / l2;
            for &(nx, ny) in &nt {
                if nx + x > k1 {
                    break;
                }
                if ny + y > k2 {
                    continue;
                }
                if dp[nx + x][ny + y] < i + 1 {
                    dp[nx + x][ny + y] = i + 1;
                    used.insert((nx + x, ny + y));
                }
            }
        }

        nt.sort_by_key(|v| v.1);
        for y in 0..=k2 {
            if (d + l2 - 1) / l2 * l2 < y * l2 {
                break;
            }
            let rem = d.saturating_sub(y * l2);
            let x = (rem + l1 - 1) / l1;
            for &(nx, ny) in &nt {
                if ny + y > k2 {
                    break;
                }
                if nx + x > k1 {
                    continue;
                }
                if dp[nx + x][ny + y] < i + 1 {
                    dp[nx + x][ny + y] = i + 1;
                    used.insert((nx + x, ny + y));
                }
            }
        }

        nt = used.into_iter().collect::<Vec<_>>();
    }

    let mut res = usize::MAX;
    for i in 0..=k1 {
        for j in 0..=k2 {
            if dp[i][j] == n {
                res = res.min(c1 * i + c2 * j);
            }
        }
    }

    println!("{}", res as i64)
}
