use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut ret = vec![vec![]; n + 1];
    for i in 0..n {
        input! {k: usize, a: [usize; k]}

        for a in a {
            ret[a].push(i + 1);
        }
    }

    for i in 1..=n {
        ret[i].sort_unstable();
        ret[i].dedup();
        println!("{} {}", ret[i].len(), ret[i].iter().join(" "));
    }
}
