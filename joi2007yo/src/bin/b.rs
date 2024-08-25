use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: [usize; 28]}

    let mut r = vec![true; 31];
    for a in a {
        r[a] = false;
    }

    println!(
        "{}",
        r.into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, f)| f.then_some(i))
            .join("\n")
    )
}
