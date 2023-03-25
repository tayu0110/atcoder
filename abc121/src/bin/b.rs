#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, c: i32, b: [i32; m], a: [[i32; m]; n]};

    let mut res = 0;
    for v in a {
        let ab = b.iter().zip(v.into_iter()).fold(0, |s, (b, a)| s + *b * a) + c;
        if ab > 0 {
            res += 1;
        }
    }

    println!("{}", res);
}
