use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, event: [(usize, usize); k]}

    let mut ac = vec![0; n];
    let mut ret = vec![];
    for (a, _) in event {
        ac[a - 1] += 1;

        if ac[a - 1] == m {
            ret.push(a);
        }
    }

    if !ret.is_empty() {
        println!("{}", ret.iter().join(" "))
    }
}
