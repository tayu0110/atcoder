#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, x: usize, a: [usize; m]}

    let mut d = vec![0; n+1];
    for v in a {
        d[v] += 1;
    }

    let r1 = d.iter().take(x).sum::<usize>();
    let r2 = d.iter().skip(x).sum::<usize>();

    println!("{}", std::cmp::min(r1, r2));
}
