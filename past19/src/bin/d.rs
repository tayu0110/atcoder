use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, t: i32, p: [(i32, i32); n]}

    let (mut sa, mut sb) = (0, 0);
    for &(a, b) in &p {
        if sa < a {
            (sa, sb) = (a, b);
        } else if sa == a && sb > b {
            (sa, sb) = (a, b);
        }
    }

    println!(
        "{}",
        p.into_iter().map(|(a, b)| t * (sa - a) + b - sb).join("\n")
    );
}
