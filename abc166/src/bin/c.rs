use proconio::*;

fn main() {
    input! {n: usize, m: usize, h: [usize; n], e: [(usize, usize); m]}
    let mut res = vec![true; n];
    for (a, b) in e.into_iter().map(|(a, b)| (a - 1, b - 1)) {
        res[a] &= h[a] > h[b];
        res[b] &= h[b] > h[a];
    }

    println!("{}", res.into_iter().filter(|&f| f).count())
}
