use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut c = [0; 30];
    for a in a {
        c[a] += 1;
    }

    println!("{}", c.iter().join(" "));
}
