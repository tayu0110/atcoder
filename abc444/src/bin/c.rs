use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    a.sort_unstable();

    let mut ret = vec![];
    if a.len() % 2 == 0 {
        let b = a[0] + a[n - 1];
        if a.iter()
            .take(a.len() / 2)
            .zip(a.iter().rev().take(a.len() / 2))
            .map(|(l, r)| l + r)
            .all(|a| a == b)
        {
            ret.push(b);
        }
    }

    let max = a[n - 1];
    while a.pop_if(|a| *a == max).is_some() {}
    if a.len() % 2 == 0 {
        if !a.is_empty() {
            let n = a.len();
            let b = a[0] + a[n - 1];
            if a.iter()
                .take(a.len() / 2)
                .zip(a.iter().rev().take(a.len() / 2))
                .map(|(l, r)| l + r)
                .all(|a| a == b)
            {
                ret.push(b);
            }
        } else {
            ret.push(max);
        }
    }

    ret.sort();
    println!("{}", ret.iter().join(" "));
}
