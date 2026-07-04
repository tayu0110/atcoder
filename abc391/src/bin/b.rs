use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Bytes; n], t: [marker::Bytes; m]}

    for i in 0..n {
        if i + m > n {
            continue;
        }
        for j in 0..n {
            if j + m > n {
                continue;
            }

            let mut bad = false;
            for k in 0..m {
                for l in 0..m {
                    if s[i + k][j + l] != t[k][l] {
                        bad = true;
                    }
                }
            }

            if !bad {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
