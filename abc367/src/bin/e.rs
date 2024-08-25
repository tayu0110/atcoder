use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, x: [usize; n], mut a: [usize; n]}

    const N: usize = 63;
    let mut d = vec![vec![0; n]; N];
    for i in 0..n {
        d[0][i] = x[i] - 1;
    }

    for i in 0..N - 1 {
        for j in 0..n {
            d[i + 1][j] = d[i][d[i][j]];
        }
    }

    let mut now = k;
    for i in (0..N).rev() {
        if now & (1 << i) != 0 {
            now -= 1 << i;
            let mut next = vec![0; n];
            for j in 0..n {
                next[j] = a[d[i][j]];
            }
            a = next;
        }
    }

    println!("{}", a.iter().join(" "))
}
