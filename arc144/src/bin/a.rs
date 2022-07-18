#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize};

    let mut m = n * 2;
    println!("{}", m);
    let v = [8, 6, 4, 2];

    let mut s = vec![];
    for w in v.iter() {
        let res = m / *w;
        m %= *w;
        for _ in 0..res {
            s.push(*w);
        }
    }

    s.reverse();
    if m == 1 {
        s[0] = 10 + s[0];
    }

    for w in s {
        print!("{}", w / 2);
    }
    println!("");
}
