use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut cur = 1;
    let mut res = vec![usize::MAX; n];
    while res.iter().any(|&r| r == usize::MAX) {
        let mut max = 0;
        for i in 0..n {
            if res[i] == usize::MAX {
                max = max.max(p[i]);
            }
        }

        let mut cnt = 0;
        for i in 0..n {
            if p[i] == max {
                res[i] = cur;
                cnt += 1;
            }
        }

        cur += cnt;
    }

    println!("{}", res.iter().join("\n"))
}
