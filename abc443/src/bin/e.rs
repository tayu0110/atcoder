use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, c: usize, mut s: [marker::Bytes; n]}
        s.reverse();

        let mut rock = vec![n; n];
        for j in 0..n {
            for i in 0..n {
                if s[i][j] == b'#' {
                    rock[j] = i;
                }
            }
        }
    }
}
