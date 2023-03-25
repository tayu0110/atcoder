use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let c = [a, b].concat();
    let mut c = c
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<Vec<_>>();
    c.sort();

    let mut res = vec![0; n + m];
    for i in 0..n + m {
        res[c[i].1] = i + 1;
    }

    println!("{}", res[0..n].to_vec().into_iter().join(" "));
    println!("{}", res[n..].to_vec().into_iter().join(" "));
}
