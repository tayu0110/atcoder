#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, mut x: usize, s: Chars}

    let mut buf = vec![];
    while x > 0 {
        buf.push(x % 2);
        x /= 2;
    }
    buf.reverse();

    for c in s {
        match c {
            'U' => { buf.pop(); }
            'L' => { buf.push(0); }
            _ => { buf.push(1); }
        }
    }

    let mut res = 0;
    for c in buf {
        res = res * 2 + c;
    }

    println!("{}", res);
}
