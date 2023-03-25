#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {v: usize, t: usize, s: usize, d: usize};

    let ns = s * v;
    let nt = t * v;
    if d < nt || ns < d {
        println!("Yes");
    } else {
        println!("No");
    }
}
