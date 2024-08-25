use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut e = e
        .into_iter()
        .map(|(_, b)| b.saturating_sub(n))
        .collect::<Vec<_>>();
    e.sort_unstable();

    println!("{}", e[..m - 1].iter().sum::<usize>())
}
