#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let v = a
        .into_iter()
        .fold(vec![0], |mut v, s| {
            let last = *v.last().unwrap();
            if s == last + 1 {
                v.push(s);
            }
            v
        });

        if v.len() == 1 {
            println!("-1");
        } else {
            println!("{}", n + 1 - v.len());
        }
}
