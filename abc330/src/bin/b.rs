use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, l: i32, r: i32, a: [i32; n]}

    println!(
        "{}",
        a.into_iter()
            .map(|a| {
                if a < l {
                    l
                } else if l <= a && a <= r {
                    a
                } else {
                    r
                }
            })
            .join(" ")
    )
}
