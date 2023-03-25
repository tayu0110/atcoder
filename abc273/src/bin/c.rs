#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [usize; n]}

    let mut b = a.clone();
    b.sort();
    b.dedup();

    let mut res = vec![0; n];
    for a in a {
        let (mut l, mut r) = (0, b.len());
        while r - l > 1 {
            let m = (r + l) / 2;
            if b[m] <= a {
                l = m;
            } else {
                r = m;
            }
        }

        res[b.len() - r] += 1;
    }

    for v in res {
        println!("{}", v);
    }
}
