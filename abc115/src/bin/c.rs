#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, mut h: [usize; n]}
    h.sort();

    let (mut l, mut r) = (0, k-1);

    let mut res = std::usize::MAX;
    while r < n {
        res = std::cmp::min(h[r]-h[l], res);
        l += 1;
        r += 1;
    }

    println!("{}", res);
}
