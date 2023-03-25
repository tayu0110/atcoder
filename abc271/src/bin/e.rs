#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(usize, usize, usize); m], e: [usize; k]}

    let mut dist = vec![std::usize::MAX; n];
    dist[0] = 0;

    for ne in e {
        let (a, b, c) = p[ne-1];

        if dist[a-1] != std::usize::MAX {
            dist[b-1] = std::cmp::min(dist[b-1], dist[a-1] + c);
        }

    }

    if dist[n-1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[n-1]);
    }
}
