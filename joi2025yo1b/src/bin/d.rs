use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    while a.len() > 1 {
        a = a.windows(2).map(|v| v[0] + v[1]).collect::<Vec<_>>();
        println!("{}", a.iter().join(" "));
    }
}
