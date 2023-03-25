#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, s: Chars, t: Chars};
    let s = s.into_iter().zip(t.into_iter()).fold(String::new(), |mut s, (l, r)| {
        s.push(l);
        s.push(r);
        s
    });

    println!("{}", s);
}
