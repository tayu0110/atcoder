use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [u32; n], b: [u32; m]}

    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<_>>();
    a.extend(b.into_iter().enumerate().map(|(j, b)| (b, j + n)));
    a.sort_unstable();

    let mut na = vec![0; n];
    let mut nb = vec![0; m];
    for (i, (_, j)) in a.into_iter().enumerate() {
        if j < n {
            na[j] = i + 1;
        } else {
            nb[j - n] = i + 1;
        }
    }

    println!("{}", na.into_iter().join(" "));
    println!("{}", nb.into_iter().join(" "))
}
