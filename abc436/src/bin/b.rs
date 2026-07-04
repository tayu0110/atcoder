use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut ret = vec![vec![0; n]; n];
    let mut k = 1;
    ret[0][(n - 1) / 2] = 1;
    let (mut r, mut c) = (0, (n - 1) / 2);
    for _ in 0..n * n - 1 {
        k += 1;
        let mut nr = (r + n - 1) % n;
        let mut nc = (c + 1) % n;
        if ret[nr][nc] != 0 {
            nr = (r + 1) % n;
            nc = c
        }
        ret[nr][nc] = k;
        (r, c) = (nr, nc);
    }

    for row in ret {
        println!("{}", row.iter().join(" "))
    }
}
