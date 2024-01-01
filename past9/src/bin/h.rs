use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let (sl, tl) = (s.len(), t.len());
    let mut memo = vec![vec![0; tl + 1]; sl + 1];
    memo[0][0] = 0;
    for i in 0..sl {
        for j in 0..tl {
            if memo[i][j] == u32::MAX {
                continue;
            }

            if s[i] != t[j] {
                memo[i + 1][j + 1] = memo[i + 1][j + 1].max(memo[i][j] + 1);
            } else {
                memo[i + 1][j + 1] = memo[i + 1][j].max(memo[i][j + 1]);
            }
        }
    }

    println!("{}", memo[sl][tl]);
}
