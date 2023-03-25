#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    let (mut l, mut r) = (0, std::usize::MAX / k);
    while r - l > 1 {
        let m = (r + l) / 2;
        let res = a.iter().map(|v| std::cmp::min(m, *v)).sum::<usize>();

        if res >= k * m {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
