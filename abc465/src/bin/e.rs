use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: marker::Bytes}

    // 0: length, 1: used number, 2: is less or equal, 3: mod 3
    let mut dp = vec![vec![[[0; 3]; 2]; 1 << 10]; n.len() + 1];
    dp[0][0][1][0] = 1;
    for (i, dig) in n.iter().map(|d| d - b'0').enumerate() {
        for j in 0..1 << 10 {
            for k in 0..2 {
                for l in 0..3 {
                    if dp[i][j][k][l] == 0 {
                        continue;
                    }

                    for m in 0..10 {
                        if k == 1 && m > dig {
                            continue;
                        }

                        let nj = if j == 0 && m == 0 { 0 } else { j | (1 << m) };
                        let nk = (k == 1 && m == dig) as usize;
                        let nl = (l * 10 + m as usize) % 3;
                        dp[i + 1][nj][nk][nl] += dp[i][j][k][l];
                        dp[i + 1][nj][nk][nl] %= M;
                    }
                }
            }
        }
    }

    let mut ret = 0;
    // only clause 1 is satisfied
    for j in 0usize..1 << 10 {
        if j & (1 << 3) != 0 {
            continue;
        }
        if j.count_ones() == 3 {
            continue;
        }
        // ignore 0
        if j == 0 {
            continue;
        }
        for k in 0..2 {
            ret += dp[n.len()][j][k][0];
            ret %= M;
        }
    }
    eprintln!("{ret}");
    // only clause 2 is satisfied
    for j in 0usize..1 << 10 {
        if j & (1 << 3) == 0 {
            continue;
        }
        if j.count_ones() == 3 {
            continue;
        }
        for k in 0..2 {
            for l in 1..3 {
                ret += dp[n.len()][j][k][l];
                ret %= M;
            }
        }
    }
    eprintln!("{ret}");
    // only clause 3 is satisfied
    for j in 0usize..1 << 10 {
        if j.count_ones() != 3 {
            continue;
        }
        if j & (1 << 3) != 0 {
            continue;
        }
        for k in 0..2 {
            for l in 1..3 {
                ret += dp[n.len()][j][k][l];
                ret %= M;
            }
        }
    }

    println!("{ret}")
}
