use proconio::*;

fn main() {
    input! {a: usize, b: usize, k: usize}

    let mut t = (a..=b)
        .take(k)
        .chain((a..=b).rev().take(k))
        .collect::<Vec<_>>();
    t.sort();
    t.dedup();

    for t in t {
        println!("{}", t)
    }
}
