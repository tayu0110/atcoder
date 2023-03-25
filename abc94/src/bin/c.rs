#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: [usize; n]}

    let mut a = x.clone();
    a.sort();

    let (m1, m2) = (a[n/2-1], a[n/2]);

    for v in x {
        if v <= m1 {
            println!("{}", m2);
        } else {
            println!("{}", m1);
        }
    }
}
