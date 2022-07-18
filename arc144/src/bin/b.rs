#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, a: i64, b: i64, p: [i64; n]};

    let mut l = 0;
    let mut r = 111222333444;
    while r - l > 1 {
        let m = (r + l) / 2;
        let p = p.clone();
        let mut na = 0;
        let mut nb = 0;
        for v in p {
            if v < m {
                let diff = m - v;
                na += (diff + a - 1) / a;
            }
            if v > m {
                let diff = v - m;
                nb += diff / b;
            }
        }

        if nb >= na {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
