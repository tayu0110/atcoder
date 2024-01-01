use itertools::Itertools;
use math::divisors_enumeration;
use proconio::*;

fn main() {
    input! {n: u64}

    let mut f = divisors_enumeration(n);
    f.sort_unstable();
    println!("{}", f.iter().join("\n"))
}
