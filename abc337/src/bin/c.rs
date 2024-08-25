use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut next = vec![usize::MAX; n];
    let mut now = 0;
    for (i, a) in a.into_iter().enumerate() {
        if a < 0 {
            now = i;
            continue;
        }

        next[a as usize - 1] = i;
    }

    let mut res = vec![];
    while now < usize::MAX {
        res.push(now + 1);
        now = next[now];
    }

    println!("{}", res.iter().join(" "))
}
