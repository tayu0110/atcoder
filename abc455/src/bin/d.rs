use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize); q]}

    let mut upper = vec![0; n + 1];
    let mut down = vec![0; n + 1];
    for (c, p) in p {
        if down[c] != 0 {
            upper[down[c]] = 0;
        }
        upper[p] = c;
        down[c] = p;
    }

    let mut cnt = vec![0; n + 1];
    for &u in &upper {
        cnt[u] += 1;
    }
    let mut ret = vec![0; n + 1];
    for i in 1..n + 1 {
        if cnt[i] == 0 {
            ret[i] = 1;
            let mut now = i;
            while upper[now] != 0 {
                ret[i] += 1;
                now = upper[now];
            }
        }
    }

    println!("{}", ret[1..].iter().join(" "))
}
