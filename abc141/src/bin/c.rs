#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, q: usize, a: [usize; q]};

    let mut v = vec![0; n];
    for a in a {
        v[a-1] += 1;
    }

    for v in v {
        if k <= q - v {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
