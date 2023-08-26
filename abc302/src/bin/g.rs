use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut b = a.clone();
    b.sort();

    let mut t = vec![vec![0; 4]; 4];
    a.iter().zip(&b).for_each(|(&a, &b)| {
        t[a][b] += 1;
    });

    let mut res = 0usize;
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                t[i][j] = 0;
                t[j][i] = 0;
                continue;
            }
            let min = t[i][j].min(t[j][i]);
            t[i][j] -= min;
            t[j][i] -= min;
            res += min;
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                continue;
            }
            for k in 0..4 {
                if i == k || j == k {
                    continue;
                }

                let min = t[i][j].min(t[j][k]).min(t[k][i]);
                t[i][j] -= min;
                t[j][k] -= min;
                t[k][i] -= min;
                res += 2 * min;
            }
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                continue;
            }
            for k in 0..4 {
                if i == k || j == k {
                    continue;
                }
                for l in 0..4 {
                    if vec![i, j, k].into_iter().any(|v| v == l) {
                        continue;
                    }

                    let min = t[i][j].min(t[j][k]).min(t[k][l]).min(t[l][i]);
                    t[i][j] -= min;
                    t[j][k] -= min;
                    t[k][l] -= min;
                    t[l][i] -= min;
                    res += 3 * min;
                }
            }
        }
    }

    println!("{}", res)
}
