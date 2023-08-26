use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n-1]}

    let mut t = vec![0; n];
    for a in a {
        t[a - 1] += 1;
    }

    println!("{}", t.iter().join("\n"))
}
