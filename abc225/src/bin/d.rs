use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut before = vec![std::usize::MAX; n];
    let mut after = vec![std::usize::MAX; n];
    let mut res = vec![];

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, y: usize}
            before[y - 1] = x - 1;
            after[x - 1] = y - 1;
        } else if t == 2 {
            input! {x: usize, y: usize}
            before[y - 1] = std::usize::MAX;
            after[x - 1] = std::usize::MAX;
        } else {
            input! {x: usize}

            let mut now = x - 1;
            while before[now] != std::usize::MAX {
                now = before[now];
            }

            let mut r = vec![];
            while now != std::usize::MAX {
                r.push(now + 1);
                now = after[now];
            }
            res.push(r);
        }
    }

    for v in res {
        println!("{} {}", v.len(), v.into_iter().join(" "))
    }
}
